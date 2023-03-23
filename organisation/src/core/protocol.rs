use std::collections::HashMap;

use crate::core::ethereum::EthereumFacade;
use crate::core::ipfs::IPFSFacade;
use color_eyre::eyre::eyre;
use log::warn;
use tokio::select;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinHandle;

use crate::errors::Result;
use crate::grpc::command::{CreatePeersetRequest, CreatePeersetResponse, Node, PermissionGraph};
use crate::ipfs::ipfs_client::CID;

/// todo: define interface for access queries.
/// for now let's start with something minimalistic for tests
pub struct ProtocolFacade {
    query_sender: tokio::sync::mpsc::Sender<QueryEvent>,
    command_sender: tokio::sync::mpsc::Sender<CommandEvent>,
}

impl ProtocolFacade {
    pub async fn query_users_in_group(
        &self,
        group_id: String,
    ) -> std::result::Result<UsersInGroupResponse, RecvError> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.query_sender
            .send(QueryEvent::QueryUsersInGroup {
                group_id,
                response_channel: sender,
            })
            .await
            .expect("should succeed");

        receiver.await
    }

    pub async fn create_peerset(
        &self,
        create_peerset_request: CreatePeersetRequest,
    ) -> std::result::Result<CreatePeersetResponse, RecvError> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.command_sender
            .send(CommandEvent::CreatePeersetRequest {
                request: create_peerset_request,
                response_channel: sender,
            })
            .await
            .expect("should succeed");

        receiver.await
    }
}

#[derive(Debug)]
pub enum IPFSEvent {
    PermissionGraphLoaded {
        cid: CID,
        permission_graph: PermissionGraph,
        peerset_address: String,
    },

    PermissionGraphSaved {
        cid: CID,
        context: CommandEvent,
    },
}

#[derive(Debug)]
pub enum BlockchainEvent {
    NewPeersetCreated {
        peers: Vec<Peer>,
        permission_graph_cid: String,
        peerset_address: String,
        context: Option<CommandEvent>,
    },
    NewChangeProposed {
        peerset_blockchain_address: String,
        proposed_by: Peer,
        new_permission_graph_cid: String,
        change_id: String,
    },
    // ChangeAccepted(),
    // ChangeDeclined(),
}

#[derive(Debug)]
enum QueryEvent {
    QueryUsersInGroup {
        group_id: String,
        response_channel: tokio::sync::oneshot::Sender<UsersInGroupResponse>,
    },
}

#[derive(Debug)]
pub struct UsersInGroupResponse {
    group_id: String,
    users: Vec<Node>,
}

#[derive(Debug)]
pub enum CommandEvent {
    CreatePeersetRequest {
        request: CreatePeersetRequest,
        response_channel: tokio::sync::oneshot::Sender<CreatePeersetResponse>,
    },
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub blockchain_address: String,
}

#[derive(Debug)]
enum PeerSetTransactionState {
    None,
    ChangeProposed {
        proposed_by: Peer,
        permission_graph: Option<PermissionGraph>,
        permission_graph_cid: CID,
        change_id: String,
    },
}

#[derive(Debug)]
struct PeerSet {
    blockchain_address: String,
    peers: Vec<Peer>,

    permission_graph_cid: CID,
    permission_graph: Option<PermissionGraph>, // todo: we probably shouldn't use type that's used in transport protocol here!

    transaction_state: PeerSetTransactionState,
}

/// A single thread that loops through possible events and processes them as they come in.
/// Entire application state is modified by this thread, all external communication is handled by other threads/tasks
/// that communicate results asynchronously with the ProtocolService.
struct ProtocolService {
    ipfs_facade: Box<dyn IPFSFacade>,
    ethereum_facade: Box<dyn EthereumFacade>,
    peersets: HashMap<String, PeerSet>,
    // todo: implement index!
}

impl ProtocolService {
    fn start_event_loop(
        mut blockchain_events_channel: tokio::sync::mpsc::Receiver<BlockchainEvent>,
        mut ipfs_events_channel: tokio::sync::mpsc::Receiver<IPFSEvent>,
        mut query_events_channel: tokio::sync::mpsc::Receiver<QueryEvent>,
        mut command_events_channel: tokio::sync::mpsc::Receiver<CommandEvent>,
        ipfs_facade: Box<dyn IPFSFacade>,
        ethereum_facade: Box<dyn EthereumFacade>,
    ) -> JoinHandle<()> {
        let mut protocol = ProtocolService {
            ipfs_facade,
            ethereum_facade,
            peersets: HashMap::new(),
        };

        let handle = tokio::spawn(async move {
            loop {
                let result = select! {
                    Some(blockchain_event) = blockchain_events_channel.recv() => {
                        protocol.handle_blockchain_event(blockchain_event)
                    }
                    Some(ipfs_event) = ipfs_events_channel.recv() => {
                        protocol.handle_ipfs_event(ipfs_event)
                    }
                    Some(query_event) = query_events_channel.recv() => {
                        protocol.handle_query_event(query_event)
                    }
                    Some(command_event) = command_events_channel.recv() => {
                        protocol.handle_command_event(command_event)
                    }
                };

                if let Err(e) = result {
                    warn!("Error occurred during processing of events: {}", e)
                }
            }
        });

        handle
    }

    fn handle_command_event(&mut self, command_event: CommandEvent) -> Result<()> {
        match &command_event {
            CommandEvent::CreatePeersetRequest {
                request:
                    CreatePeersetRequest {
                        initial_permission_graph,
                        ..
                    },
                ..
            } => {
                self.ipfs_facade.async_save_permission_graph(
                    initial_permission_graph.clone().unwrap(),
                    command_event,
                );
            }
        }

        Ok(())
    }

    fn handle_query_event(&mut self, query_event: QueryEvent) -> Result<()> {
        // todo: this is scrappy for now to be able to unit test what's currently implemented.
        match query_event {
            QueryEvent::QueryUsersInGroup {
                group_id,
                response_channel,
            } => {
                let mut users = vec![];
                for (_, peerset) in &self.peersets {
                    match peerset.permission_graph {
                        Some(ref permission_graph) => {
                            for (_, edges) in &permission_graph.edges {
                                if let Some(ref source) = &edges.source {
                                    for edge in &edges.edges {
                                        if edge.destination_node_id == group_id {
                                            users.push(source.clone())
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                response_channel
                    .send(UsersInGroupResponse { group_id, users })
                    .expect("Sending should succeed");
            }
        }

        Ok(())
    }

    fn handle_blockchain_event(&mut self, blockchain_event: BlockchainEvent) -> Result<()> {
        match blockchain_event {
            BlockchainEvent::NewPeersetCreated {
                peers,
                permission_graph_cid,
                peerset_address,
                context,
            } => {
                let address = peerset_address;
                let cid = permission_graph_cid;

                self.peersets.insert(
                    address.clone(),
                    PeerSet {
                        blockchain_address: address.clone(),
                        peers,
                        permission_graph_cid: cid.clone(),
                        permission_graph: None,
                        transaction_state: PeerSetTransactionState::None,
                    },
                );

                self.ipfs_facade
                    .async_load_permission_graph(cid, address.clone());

                if let Some(context) = context {
                    match context {
                        CommandEvent::CreatePeersetRequest {
                            response_channel, ..
                        } => {
                            response_channel
                                .send(CreatePeersetResponse {
                                    deployed_peerset_smart_contract_address: address,
                                })
                                .expect("sending should succeed");
                        }
                    }
                }
            }

            BlockchainEvent::NewChangeProposed {
                peerset_blockchain_address,
                proposed_by,
                new_permission_graph_cid,
                change_id,
            } => {
                let peerset = self.peerset_by_address(peerset_blockchain_address.as_str())?;

                match &peerset.transaction_state {
                    PeerSetTransactionState::None => {
                        peerset.transaction_state = PeerSetTransactionState::ChangeProposed {
                            proposed_by,
                            permission_graph: None,
                            permission_graph_cid: new_permission_graph_cid.clone(),
                            change_id,
                        }
                    }
                    _ => {
                        return Err(eyre!(
                            "PeerSetTransactionState should be None when a new change is proposed."
                        ))
                    }
                }

                // download permission graph
                self.ipfs_facade.async_load_permission_graph(
                    new_permission_graph_cid,
                    peerset_blockchain_address,
                );
            }
        }

        Ok(())
    }

    fn handle_ipfs_event(&mut self, ipfs_event: IPFSEvent) -> Result<()> {
        match ipfs_event {
            IPFSEvent::PermissionGraphLoaded {
                cid: cid_loaded,
                permission_graph: loaded_permission_graph,
                peerset_address,
            } => {
                let peerset = self.peerset_by_address(peerset_address.as_str())?;

                if cid_loaded == peerset.permission_graph_cid {
                    peerset.permission_graph = Some(loaded_permission_graph);
                    // todo: update index here
                    return Ok(());
                }

                match &mut peerset.transaction_state {
                    PeerSetTransactionState::ChangeProposed {
                        proposed_by: _, permission_graph, permission_graph_cid: new_permission_graph_cid, change_id: _
                    } => {
                        if new_permission_graph_cid == &cid_loaded {
                            // todo: at this point we can do some processing and vote whether change should be accepted or rejected.
                            *permission_graph = Some(loaded_permission_graph);
                        } else {
                            return Err(eyre!("Unknown CID {} loaded for peerset {}, peerset: {:?}", cid_loaded, peerset_address, peerset));
                        }
                    }
                    _ => {
                        return Err(eyre!("PeerSetTransactionState should be ChangeProposed after ipfs graph has been loaded."))
                    }
                }
            }
            IPFSEvent::PermissionGraphSaved { cid, context } => match &context {
                CommandEvent::CreatePeersetRequest {
                    request:
                        CreatePeersetRequest {
                            name: _name, peers, ..
                        },
                    ..
                } => {
                    let peers = peers
                        .into_iter()
                        .map(|e| Peer {
                            blockchain_address: e.to_string(),
                        })
                        .collect();

                    self.ethereum_facade
                        .async_create_peerset(peers, cid, context)
                }
            },
        }

        Ok(())
    }

    fn peerset_by_address(&mut self, address: &str) -> Result<&mut PeerSet> {
        return self
            .peersets
            .get_mut(address)
            .ok_or(eyre!("no peerset with given address"));
    }
}

// todo test protocol struct!
#[cfg(test)]
mod tests {
    use crate::core::ethereum::EthereumFacade;
    use crate::core::ipfs::IPFSFacade;
    use crate::core::protocol::{
        BlockchainEvent, CommandEvent, IPFSEvent, Peer, ProtocolFacade, ProtocolService,
    };
    use crate::grpc::command::{
        CreatePeersetRequest, Edge, Edges, Node, NodeType, PermissionGraph,
    };
    use std::collections::HashMap;

    use crate::ipfs::ipfs_client::CID;

    static PEER_ONE_ADDR: &'static str = "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b";
    static PEER_TWO_ADDR: &'static str = "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e";
    static PEERSET_ADDR: &'static str = "0xd248e4a8407ed7ff9bdbc396ba46723b8101c32f";

    #[tokio::test]
    async fn blockchain_event_test() {
        // given
        let (blockchain_sender, blockchain_receiver) = tokio::sync::mpsc::channel(100);

        let (ipfs_sender, ipfs_receiver) = tokio::sync::mpsc::channel(100);

        let (query_sender, query_receiver) = tokio::sync::mpsc::channel(100);

        let (command_sender, command_receiver) = tokio::sync::mpsc::channel(100);

        let ipfs_facade = IPFSFacadeMock {
            sender: ipfs_sender.clone(),
        };

        let ethereum_facade = EthereumFacadeMock {
            sender: blockchain_sender.clone(),
        };

        let _handle = ProtocolService::start_event_loop(
            blockchain_receiver,
            ipfs_receiver,
            query_receiver,
            command_receiver,
            Box::new(ipfs_facade),
            Box::new(ethereum_facade),
        );

        let protocol_facade = ProtocolFacade {
            command_sender,
            query_sender,
        };

        // when
        let response = protocol_facade
            .create_peerset(CreatePeersetRequest {
                name: "p1".to_string(),
                peers: vec![PEER_ONE_ADDR.to_string(), PEER_TWO_ADDR.to_string()],
                initial_permission_graph: Some(test_graph_with_user_and_group()),
            })
            .await
            .expect("should succeed");

        // then
        assert_eq!(
            response.deployed_peerset_smart_contract_address,
            PEERSET_ADDR
        );

        // then
        let response = protocol_facade
            .query_users_in_group("gr_1".to_string())
            .await
            .expect("response should be ready");

        assert_eq!(response.users.len(), 1);
        assert_eq!(response.users[0].id, "ur_1".to_string());
    }

    pub struct IPFSFacadeMock {
        pub sender: tokio::sync::mpsc::Sender<IPFSEvent>,
    }

    pub struct EthereumFacadeMock {
        pub sender: tokio::sync::mpsc::Sender<BlockchainEvent>,
    }

    impl EthereumFacade for EthereumFacadeMock {
        fn async_create_peerset(
            &self,
            peers: Vec<Peer>,
            permission_graph_cid: CID,
            context: CommandEvent,
        ) {
            self.sender
                .try_send(BlockchainEvent::NewPeersetCreated {
                    peers,
                    permission_graph_cid,
                    peerset_address: PEERSET_ADDR.to_string(),
                    context: Some(context),
                })
                .expect("should succeed");
        }
    }

    impl IPFSFacade for IPFSFacadeMock {
        fn async_load_permission_graph(&self, cid: CID, peerset_address: String) {
            self.sender
                .try_send(IPFSEvent::PermissionGraphLoaded {
                    cid,
                    permission_graph: test_graph_with_user_and_group(),
                    peerset_address,
                })
                .expect("should succeed");
        }

        fn async_save_permission_graph(&self, _cid: PermissionGraph, context: CommandEvent) {
            let sender = self.sender.clone();
            sender
                .try_send(IPFSEvent::PermissionGraphSaved {
                    cid: "ipfs://test-cid-1".to_string(),
                    context,
                })
                .expect("should succeed");
        }
    }

    fn test_graph_with_user_and_group() -> PermissionGraph {
        return PermissionGraph {
            edges: HashMap::from([
                (
                    "ur_1".to_string(),
                    Edges {
                        source: Some(Node {
                            id: "ur_1".to_string(),
                            r#type: NodeType::User as i32,
                            peerset_address: None,
                        }),
                        edges: vec![Edge {
                            destination_node_id: "gr_1".to_string(),
                            permission: "belongs".to_string(),
                        }],
                    },
                ),
                (
                    "gr_1".to_string(),
                    Edges {
                        source: Some(Node {
                            id: "gr_1".to_string(),
                            r#type: NodeType::User as i32,
                            peerset_address: None,
                        }),
                        edges: vec![],
                    },
                ),
            ]),
        };
    }
}

use std::collections::HashMap;

use crate::core::ethereum::{EthereumFacade, EthereumFacadeImpl};
use crate::core::ipfs::{CheatingIPFSFacade, IPFSFacade};
use crate::core::protocol::BlockchainEvent::NewPeersetCreated;
use color_eyre::eyre::eyre;
use ethers_signers::LocalWallet;
use itertools::Itertools;
use log::info;
use tokio::select;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinHandle;

use crate::errors::Result;
use crate::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse, Node, PeersetCreatedRequest, PeersetGraph,
    PermissionGraph, ProposeChangeRequest, ProposeChangeResponse, QueryPeersetsCiDsRequest,
    QueryPeersetsCiDsResponse,
};
use crate::ipfs::ipfs_client::CID;

/// todo: define interface for access queries.
/// todo: define interface for commands
pub struct ProtocolFacade {
    query_sender: tokio::sync::mpsc::Sender<QueryEvent>,
    command_sender: tokio::sync::mpsc::Sender<CommandEvent>,
    blockchain_sender: tokio::sync::mpsc::Sender<BlockchainEvent>,
}

impl ProtocolFacade {
    pub fn new(wallet: LocalWallet) -> Self {
        let (blockchain_sender, blockchain_receiver) = tokio::sync::mpsc::channel(100);
        let (ipfs_sender, ipfs_receiver) = tokio::sync::mpsc::channel(100);
        let (query_sender, query_receiver) = tokio::sync::mpsc::channel(100);
        let (command_sender, command_receiver) = tokio::sync::mpsc::channel(100);

        let ipfs_facade = CheatingIPFSFacade::new(ipfs_sender);
        let ethereum_facade = EthereumFacadeImpl::new(wallet, blockchain_sender.clone());

        let _handle = ProtocolService::start_event_loop(
            blockchain_receiver,
            ipfs_receiver,
            query_receiver,
            command_receiver,
            Box::new(ipfs_facade),
            Box::new(ethereum_facade),
        );

        ProtocolFacade {
            command_sender,
            query_sender,
            blockchain_sender,
        }
    }

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

    pub async fn peerset_created(&self, peerset_created: PeersetCreatedRequest) {
        self.blockchain_sender
            .try_send(NewPeersetCreated {
                peers: peerset_created
                    .peers
                    .into_iter()
                    .map(|e| Peer {
                        blockchain_address: e,
                    })
                    .collect_vec(),
                permission_graph_cid: peerset_created.permission_graph_cid,
                peerset_address: peerset_created.deployed_peerset_smart_contract_address,
                context: None,
            })
            .expect("should succeed");
    }

    pub async fn propose_change(&self, request: ProposeChangeRequest) -> ProposeChangeResponse {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.command_sender
            .send(CommandEvent::ProposeChange {
                request,
                response_channel: sender,
            })
            .await
            .unwrap();

        receiver.await.unwrap()
    }

    pub async fn query_peersets(
        &self,
        query_peersets: QueryPeersetsCiDsRequest,
    ) -> QueryPeersetsCiDsResponse {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.query_sender
            .send(QueryEvent::QueryPeersets {
                request: query_peersets,
                response_channel: sender,
            })
            .await
            .expect("should succeed");

        receiver.await.unwrap()
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

    QueryPeersets {
        request: QueryPeersetsCiDsRequest,
        response_channel: tokio::sync::oneshot::Sender<QueryPeersetsCiDsResponse>,
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
    ProposeChange {
        request: ProposeChangeRequest,
        response_channel: tokio::sync::oneshot::Sender<ProposeChangeResponse>,
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
        votes: i32,
        proposed_by: Peer,
        permission_graph: Option<PermissionGraph>,
        permission_graph_cid: CID,
        change_id: String,
        response_channel: Option<tokio::sync::oneshot::Sender<ProposeChangeResponse>>,
    },
}

#[derive(Debug)]
struct PeerSet {
    blockchain_address: String,
    peers: Vec<Peer>,

    permission_graph_cid: CID,
    permission_graph: Option<PermissionGraph>, // todo: we probably shouldn't use type that's used in transport protocol here!

    transaction_state: PeerSetTransactionState,
    // todo: here we also need to store local state (for example command: propose change)
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
                        info!("Handling event: {:?}", blockchain_event);
                        protocol.handle_blockchain_event(blockchain_event)
                    }
                    Some(ipfs_event) = ipfs_events_channel.recv() => {
                        info!("Handling event: {:?}", ipfs_event);
                        protocol.handle_ipfs_event(ipfs_event)
                    }
                    Some(query_event) = query_events_channel.recv() => {
                        info!("Handling event: {:?}", query_event);
                        protocol.handle_query_event(query_event)
                    }
                    Some(command_event) = command_events_channel.recv() => {
                        info!("Handling event: {:?}", command_event);
                        protocol.handle_command_event(command_event)
                    }
                };

                if let Err(e) = result {
                    panic!("Error occurred during processing of events: {}", e)
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
            CommandEvent::ProposeChange {
                request,
                response_channel,
            } => {
                // need to save this guy to ipfs first, ehh and only then propose a change.
                // todo: refactor to save context in the event loop
                todo!();
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
            QueryEvent::QueryPeersets {
                request: _request,
                response_channel,
            } => {
                let peerset_graphs = self
                    .peersets
                    .iter()
                    .map(|(addr, peerset)| PeersetGraph {
                        peerset_address: addr.clone(),
                        permission_graph_cid: peerset.permission_graph_cid.clone(),
                    })
                    .collect_vec();

                response_channel
                    .send(QueryPeersetsCiDsResponse { peerset_graphs })
                    .unwrap()
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
                    .async_load_permission_graph(cid.clone(), address.clone());

                if let Some(context) = context {
                    match context {
                        CommandEvent::CreatePeersetRequest {
                            response_channel, ..
                        } => {
                            response_channel
                                .send(CreatePeersetResponse {
                                    cid,
                                    deployed_peerset_smart_contract_address: address,
                                })
                                .expect("sending should succeed");
                        }
                        CommandEvent::ProposeChange { .. } => {
                            todo!()
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
                            votes: 0,
                            proposed_by,
                            permission_graph: None,
                            permission_graph_cid: new_permission_graph_cid.clone(),
                            change_id,
                            response_channel: None,
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
                        votes, proposed_by: _, permission_graph, permission_graph_cid: new_permission_graph_cid, change_id: _, response_channel
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
                _ => {
                    todo!()
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
    use crate::grpc::command::{CreatePeersetRequest, PermissionGraph};

    use crate::ipfs::ipfs_client::CID;
    use crate::poc::shared;

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
            blockchain_sender,
        };

        // when
        let response = protocol_facade
            .create_peerset(CreatePeersetRequest {
                name: "p1".to_string(),
                peers: vec![PEER_ONE_ADDR.to_string(), PEER_TWO_ADDR.to_string()],
                initial_permission_graph: Some(shared::demo_graph()),
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

        fn async_propose_change(&self, peerset_address: String, permission_graph_cid: CID) {
            todo!()
        }
    }

    impl IPFSFacade for IPFSFacadeMock {
        fn async_load_permission_graph(&self, cid: CID, peerset_address: String) {
            self.sender
                .try_send(IPFSEvent::PermissionGraphLoaded {
                    cid,
                    permission_graph: shared::demo_graph(),
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
}

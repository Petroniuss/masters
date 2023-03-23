use std::collections::HashMap;

use crate::core::ipfs::IPFSFacade;
use color_eyre::eyre::eyre;
use log::warn;
use tokio::select;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinHandle;

use crate::errors::Result;
use crate::grpc::command::{Node, PermissionGraph};
use crate::ipfs::ipfs_client::CID;

/// todo: define interface for access queries.
/// for now let's start with something minimalistic for tests
pub struct ProtocolFacade {
    protocol_sender: tokio::sync::mpsc::Sender<QueryEvent>,
}

impl ProtocolFacade {
    pub async fn query_users_in_group(
        &self,
        group_id: String,
    ) -> std::result::Result<UsersInGroupResponse, RecvError> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.protocol_sender
            .send(QueryEvent::QueryUsersInGroup {
                group_id,
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
    }, // LoadingFailureEvent if cannot read the file.
}

#[derive(Debug)]
enum BlockchainEvent {
    NewPeersetCreated {
        peers: Vec<Peer>,
        permission_graph_cid: String,
        peerset_address: String,
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

// enum CoordinatorEvents { }

#[derive(Clone, Debug)]
struct Peer {
    blockchain_address: String,
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
    peersets: HashMap<String, PeerSet>,
    // todo: implement index!
}

impl ProtocolService {
    fn start_event_loop(
        mut blockchain_events_channel: tokio::sync::mpsc::Receiver<BlockchainEvent>,
        mut ipfs_events_channel: tokio::sync::mpsc::Receiver<IPFSEvent>,
        mut query_events_channel: tokio::sync::mpsc::Receiver<QueryEvent>,
        ipfs_facade: Box<dyn IPFSFacade>,
    ) -> JoinHandle<()> {
        let mut protocol = ProtocolService {
            ipfs_facade,
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
                };

                if let Err(e) = result {
                    warn!("Error occurred during processing of events: {}", e)
                }
            }
        });

        handle
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

                self.ipfs_facade.load_permission_graph(cid, address)
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
                self.ipfs_facade
                    .load_permission_graph(new_permission_graph_cid, peerset_blockchain_address);
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
    use crate::core::ipfs::IPFSFacade;
    use crate::core::protocol::{
        BlockchainEvent, IPFSEvent, Peer, ProtocolFacade, ProtocolService,
    };
    use crate::grpc::command::{Edge, Edges, Node, NodeType, PermissionGraph};
    use std::collections::HashMap;
    use std::time::Duration;

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

        let ipfs_facade = IPFSFacadeMock {
            sender: ipfs_sender,
        };

        let handle = ProtocolService::start_event_loop(
            blockchain_receiver,
            ipfs_receiver,
            query_receiver,
            Box::new(ipfs_facade),
        );

        let protocol_facade = ProtocolFacade {
            protocol_sender: query_sender,
        };

        // when
        blockchain_sender
            .send(BlockchainEvent::NewPeersetCreated {
                peers: vec![
                    Peer {
                        blockchain_address: PEER_ONE_ADDR.to_string(),
                    },
                    Peer {
                        blockchain_address: PEER_TWO_ADDR.to_string(),
                    },
                ],
                permission_graph_cid: "ipfs://test-cid".to_string(),
                peerset_address: PEERSET_ADDR.to_string(),
            })
            .await
            .expect("should succeed");

        // wait for events to be processed
        // todo: figure out a smarter way to make sure that all events have been processed.
        tokio::time::sleep(Duration::from_micros(10)).await;

        // let's send a couple of events and see whether it works.
        let response = protocol_facade
            .query_users_in_group("gr_1".to_string())
            .await
            .expect("response should be ready");

        assert_eq!(response.users.len(), 1);
        assert_eq!(response.users[0].id, "ur_1".to_string());

        handle.abort()
    }

    pub struct IPFSFacadeMock {
        pub sender: tokio::sync::mpsc::Sender<IPFSEvent>,
    }

    impl IPFSFacade for IPFSFacadeMock {
        fn load_permission_graph(&self, cid: CID, peerset_address: String) {
            let sender = self.sender.clone();
            tokio::spawn(async move {
                sender
                    .send(IPFSEvent::PermissionGraphLoaded {
                        cid,
                        permission_graph: test_graph_with_user_and_group(),
                        peerset_address,
                    })
                    .await
                    .expect("should succeed")
            });
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

// 1. create abstract blockchain events

// 2. connect IPFS to the protocol:
//    - for now in a scrappy way just respond with specified graph events.

// 2. connect blockchain to the protocol

// 3. connect to coordinator events

// 4. respond to coordinator events

// 5. Core protocol may also schedule other tasks to be performed asynchronously

use std::collections::HashMap;
use color_eyre::eyre::eyre;
use log::warn;
use tokio::select;
use tokio::task::JoinHandle;
use crate::core::ipfs::IPFSFacade;

use crate::grpc::command::PermissionGraph;
use crate::ipfs::ipfs_client::CID;
use crate::errors::Result;

struct ProtocolFacade {}

#[derive(Debug)]
pub enum IPFSEvent {
    PermissionGraphLoaded {
        cid: CID,
        permission_graph: PermissionGraph, // loaded or not yet.
        peerset_address: String
    }
    // LoadingFailureEvent if cannot read the file.
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

// enum CoordinatorEvents { }

#[derive(Clone, Debug)]
struct Peer {
    blockchain_address: String
}

#[derive(Debug)]
enum PeerSetTransactionState {
    None,
    ChangeProposed {
        proposed_by: Peer,
        permission_graph: Option<PermissionGraph>,
        permission_graph_cid: CID,
        change_id: String,
    }
}

#[derive(Debug)]
struct PeerSet {
    blockchain_address: String,
    peers: Vec<Peer>,

    permission_graph_cid: CID,
    permission_graph: Option<PermissionGraph>, // todo: we probably shouldn't use type that's used in transport protocol here!

    transaction_state: PeerSetTransactionState
}

/// A single thread that loops through possible events and processes them as they come in.
/// Entire application state is modified by this thread, all external communication is handled by other threads/tasks
/// that communicate results asynchronously with the ProtocolService.
struct ProtocolService {
    ipfs_facade: Box<dyn IPFSFacade>,
    peersets: HashMap<String, PeerSet>,
    storage: HashMap<CID, PermissionGraph>,

    // todo: implement index!
}

impl ProtocolService {
    fn start_event_loop(
        mut blockchain_events_channel: tokio::sync::mpsc::Receiver<
            BlockchainEvent,
        >,
        mut ipfs_events_channel: tokio::sync::mpsc::Receiver<
            IPFSEvent,
        >,
        ipfs_facade: Box<dyn IPFSFacade>
    ) -> JoinHandle<()> {
        let mut protocol = ProtocolService {
            ipfs_facade,
            peersets: HashMap::new(),
            storage: HashMap::new(),
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
                };

                if let Err(e) = result {
                    warn!("Error occurred during processing of events: {}", e)
                }
            };
        });

        handle
    }

    fn handle_blockchain_event(&mut self , blockchain_event: BlockchainEvent) -> Result<()> {
        match blockchain_event {
            BlockchainEvent::NewPeersetCreated { peers, permission_graph_cid, peerset_address } => {
                let address = peerset_address;
                let cid = permission_graph_cid;

                self.peersets.insert(address.clone(), PeerSet {
                    blockchain_address: address.clone(),
                    peers,
                    permission_graph_cid: cid.clone(),
                    permission_graph: None,
                    transaction_state: PeerSetTransactionState::None,
                });

                self.ipfs_facade.load_permission_graph(cid, address)
            }

            BlockchainEvent::NewChangeProposed {
                peerset_blockchain_address, proposed_by, new_permission_graph_cid, change_id
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
                        return Err(eyre!("PeerSetTransactionState should be None when a new change is proposed."))
                    }
                }

                // download permission graph
                self.ipfs_facade.load_permission_graph(
                    new_permission_graph_cid,
                    peerset_blockchain_address
                );
            }
        }

        Ok(())
    }

    fn handle_ipfs_event(&mut self, ipfs_event: IPFSEvent) -> Result<()> {
        match ipfs_event {
            IPFSEvent::PermissionGraphLoaded {
                cid: cid_loaded, permission_graph: loaded_permission_graph, peerset_address
            } => {
                let peerset = self.peerset_by_address(peerset_address.as_str())?;

                if cid_loaded == peerset.permission_graph_cid {
                    peerset.permission_graph = Some(loaded_permission_graph);
                    // todo: update index here
                    return Ok(())
                }


                match &mut peerset.transaction_state {
                    PeerSetTransactionState::ChangeProposed {
                        proposed_by, permission_graph, permission_graph_cid: new_permission_graph_cid, change_id
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
        return self.peersets
            .get_mut(address)
            .ok_or(eyre!("no peerset with given address"));
    }
}

// todo test protocol struct!
#[cfg(test)]
mod tests {
    use crate::core::ipfs::{IPFSFacade, IPFSFacadeMock};
    use crate::core::protocol::ProtocolService;

    #[tokio::test]
    async fn blockchain_event_test() {
        let (blockchain_sender, mut blockchain_receiver) =
            tokio::sync::mpsc::channel(100);

        let (ipfs_sender, mut ipfs_receiver) =
            tokio::sync::mpsc::channel(100);

        let ipfs_facade = IPFSFacadeMock { sender: ipfs_sender };

        let handle = ProtocolService::start_event_loop(
            blockchain_receiver,
            ipfs_receiver,
            Box::new(ipfs_facade)
        );




        handle.abort()
    }
}

// 1. create abstract blockchain events

// 2. connect IPFS to the protocol:
//    - for now in a scrappy way just respond with specified graph events.

// 2. connect blockchain to the protocol

// 3. connect to coordinator events

// 4. respond to coordinator events

// 5. Core protocol may also schedule other tasks to be performed asynchronously

use std::collections::HashMap;

use crate::core::ethereum::{AddressToString, EthereumFacade, EthereumFacadeImpl};
use crate::core::ipfs::{CheatingIPFSFacade, IPFSFacade, CID};
use crate::errors::Result;
use crate::transport::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse, Node, PeersetCreatedRequest, PeersetGraph,
    PermissionGraph, ProposeChangeRequest, ProposeChangeResponse, QueryPeersetsCiDsRequest,
    QueryPeersetsCiDsResponse,
};
use color_eyre::eyre::eyre;
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use itertools::Itertools;
use log::info;
use tokio::select;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinHandle;

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

        let peer = Peer::new(wallet.address().to_full_string());

        let ipfs_facade = CheatingIPFSFacade::new(ipfs_sender);
        let ethereum_facade = EthereumFacadeImpl::new(wallet, blockchain_sender.clone());

        let _handle = ProtocolService::start_event_loop(
            peer,
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

    pub async fn peerset_created(&self, request: PeersetCreatedRequest) {
        self.blockchain_sender
            .try_send(BlockchainEvent::NewPeersetCreated {
                peers: Peer::from_vec(&request.peers),
                permission_graph_cid: request.permission_graph_cid,
                peerset_address: request.deployed_peerset_smart_contract_address,
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
        peerset_address: Option<String>, // if None, then it's a new peerset
    },
}

#[derive(Debug)]
pub enum BlockchainEvent {
    NewPeersetCreated {
        peers: Vec<Peer>,
        permission_graph_cid: String,
        peerset_address: String,
    },
    NewChangeProposed {
        peerset_blockchain_address: String,
        proposed_by: Peer,
        new_permission_graph_cid: String,
    },
    ChangeAccepted {
        peerset_address: String,
        new_permission_graph_cid: String,
    },
    // todo: ChangeRejected
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

impl Peer {
    pub fn new(blockchain_address: String) -> Self {
        Peer { blockchain_address }
    }

    pub fn from_vec(blockchain_addresses: &Vec<String>) -> Vec<Peer> {
        blockchain_addresses
            .into_iter()
            .map(|e| Peer::new(e.clone()))
            .collect_vec()
    }

    pub fn from_address(address: &Address) -> Peer {
        Peer::new(address.to_full_string())
    }
}

#[derive(Debug)]
enum PeerSetTransactionState {
    None,
    ChangeProposed {
        votes: i32,
        proposed_by: Peer,
        permission_graph: Option<PermissionGraph>,
        permission_graph_cid: CID,
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
    // todo: need to split stuff that we access through shared reference and stuff that we access through mutable reference
    // otherwise borrow checker will break hell loose on us
    peersets: HashMap<String, PeerSet>,
    pending_command: Option<CommandEvent>,
    peer: Peer,
}

impl ProtocolService {
    fn start_event_loop(
        peer: Peer,
        mut blockchain_events_channel: tokio::sync::mpsc::Receiver<BlockchainEvent>,
        mut ipfs_events_channel: tokio::sync::mpsc::Receiver<IPFSEvent>,
        mut query_events_channel: tokio::sync::mpsc::Receiver<QueryEvent>,
        mut command_events_channel: tokio::sync::mpsc::Receiver<CommandEvent>,
        ipfs_facade: Box<dyn IPFSFacade>,
        ethereum_facade: Box<dyn EthereumFacade>,
    ) -> JoinHandle<()> {
        let mut protocol = ProtocolService {
            peer,
            ipfs_facade,
            ethereum_facade,
            peersets: HashMap::new(),
            pending_command: None,
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
        if self.pending_command.is_some() {
            // todo return error back to the caller through channel is command_event
            return Err(eyre!("Another command is already pending"));
        }

        match &command_event {
            CommandEvent::CreatePeersetRequest {
                request:
                    CreatePeersetRequest {
                        initial_permission_graph,
                        ..
                    },
                ..
            } => {
                self.ipfs_facade
                    .async_save_permission_graph(initial_permission_graph.clone().unwrap(), None);
            }
            CommandEvent::ProposeChange {
                request,
                response_channel: _,
            } => {
                // todo check if there's already a pending change
                self.ipfs_facade.async_save_permission_graph(
                    request.new_permission_graph.as_ref().unwrap().clone(),
                    Some(request.peerset_address.clone()),
                );
            }
        }

        self.pending_command = Some(command_event);
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

                self.ethereum_facade
                    .subscribe_to_peerset_events(address.clone());

                if let Some(context) = self.pending_command.take() {
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
            } => {
                let peerset = self.peerset_by_address(peerset_blockchain_address.as_str())?;
                match &peerset.transaction_state {
                    PeerSetTransactionState::None => {
                        peerset.transaction_state = PeerSetTransactionState::ChangeProposed {
                            votes: 1,
                            proposed_by,
                            permission_graph: None,
                            permission_graph_cid: new_permission_graph_cid.clone(),
                        }
                    }

                    _ => {
                        return Err(eyre!(
                            "PeerSetTransactionState should be None when a new change is proposed."
                        ))
                    }
                }

                self.ipfs_facade.async_load_permission_graph(
                    new_permission_graph_cid,
                    peerset_blockchain_address,
                );
            }
            BlockchainEvent::ChangeAccepted {
                ref peerset_address,
                ref new_permission_graph_cid,
            } => {
                let peerset = self.peerset_by_address(peerset_address)?;
                peerset.permission_graph_cid = new_permission_graph_cid.clone();
                if let Some(permission_graph) = peerset.permission_graph.take() {
                    peerset.permission_graph = Some(permission_graph)
                } else {
                    peerset.permission_graph = None;
                }
                peerset.transaction_state = PeerSetTransactionState::None;

                if let Some(command) = self.pending_command.take() {
                    match command {
                        CommandEvent::ProposeChange {
                            request: _,
                            response_channel,
                        } => response_channel
                            .send(ProposeChangeResponse {
                                proposed_cid: new_permission_graph_cid.clone(),
                                accepted: true,
                            })
                            .unwrap(),

                        CommandEvent::CreatePeersetRequest { .. } => {
                            panic!()
                        }
                    }
                }
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
                let peer_addr = self.peer.blockchain_address.clone();
                let peerset = self.peerset_by_address(peerset_address.as_str())?;
                let peerset_address = peerset.blockchain_address.clone();

                if cid_loaded == peerset.permission_graph_cid {
                    peerset.permission_graph = Some(loaded_permission_graph);
                    return Ok(());
                }

                match &mut peerset.transaction_state {
                    PeerSetTransactionState::ChangeProposed {
                        votes: _, proposed_by, permission_graph, permission_graph_cid: new_permission_graph_cid,
                    } => {
                        info!("Reached {} {}", new_permission_graph_cid, cid_loaded);
                        info!("Peer_addr {}, proposed_by {}", peer_addr, proposed_by.blockchain_address);
                        if new_permission_graph_cid == &cid_loaded {
                            *permission_graph = Some(loaded_permission_graph);

                            if peer_addr != proposed_by.blockchain_address {
                                self.ethereum_facade
                                    .async_approve_change(peerset_address.clone(), cid_loaded);
                            }
                        } else {
                            return Err(eyre!("Unknown CID {} loaded for peerset {}", cid_loaded, peerset_address));
                        }
                    }
                    _ => {
                        return Err(eyre!("PeerSetTransactionState should be ChangeProposed after ipfs graph has been loaded."))
                    }
                }
            }
            IPFSEvent::PermissionGraphSaved {
                cid,
                // todo: why is this not used?
                peerset_address: _,
            } => {
                if let Some(command) = self.pending_command.take() {
                    match command {
                        CommandEvent::CreatePeersetRequest { ref request, .. } => {
                            let peers = Peer::from_vec(&request.peers);
                            self.ethereum_facade.async_create_peerset(peers, cid);
                            self.pending_command = Some(command);
                        }
                        CommandEvent::ProposeChange { ref request, .. } => {
                            self.ethereum_facade
                                .async_propose_change(request.peerset_address.clone(), cid);

                            self.pending_command = Some(command);
                        }
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
    use crate::core::ethereum::EthereumFacade;
    use crate::core::ipfs::{IPFSFacade, CID};
    use crate::core::protocol::{
        BlockchainEvent, IPFSEvent, Peer, ProtocolFacade, ProtocolService,
    };
    use crate::transport::grpc::command::{CreatePeersetRequest, PermissionGraph};

    use crate::shared::shared;

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

        let peer = Peer::new(PEER_ONE_ADDR.to_string());

        let _handle = ProtocolService::start_event_loop(
            peer,
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
        fn async_create_peerset(&self, peers: Vec<Peer>, permission_graph_cid: CID) {
            self.sender
                .try_send(BlockchainEvent::NewPeersetCreated {
                    peers,
                    permission_graph_cid,
                    peerset_address: PEERSET_ADDR.to_string(),
                })
                .expect("should succeed");
        }

        fn async_propose_change(&self, _peerset_address: String, _permission_graph_cid: CID) {}

        fn async_approve_change(&self, _peerset_address: String, _permission_graph_cid: CID) {}

        fn subscribe_to_peerset_events(&self, _peerset_address: String) {}
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

        fn async_save_permission_graph(
            &self,
            _cid: PermissionGraph,
            peerset_address: Option<String>,
        ) {
            let sender = self.sender.clone();
            sender
                .try_send(IPFSEvent::PermissionGraphSaved {
                    cid: "ipfs://test-cid-1".to_string(),
                    peerset_address,
                })
                .expect("should succeed");
        }
    }
}

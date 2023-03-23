use ethers::types::Address;
use grpc::command::organisation_dev_server::OrganisationDev;

use log::{info, warn};
use organisation::data_model::peer_set::{Peer, PeerSet};
use organisation::errors::Result;
use organisation::grpc;

use organisation::grpc::command::organisation_dev_server::OrganisationDevServer;
use organisation::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse, PeersetCreatedRequest, PeersetCreatedResponse,
    ProposeChangeRequest, ProposeChangeResponse,
};
use organisation::on_chain::ethereum_client::EnrichedEthereumClient;
use organisation::on_chain::peer_broadcast_sc::PeerBroadcastService;
use organisation::on_chain::peer_set_sc::{
    PeerSetSmartContractService, PeerSetSmartContractServiceFromAddress,
};
use organisation::poc::shared::{
    create_demo_client, demo_graph_ipfs_pointer, demo_organisation_one, shared_init,
};
use std::sync::{Arc, Mutex};

use organisation::ipfs::ipfs_client::IPFSClientFacade;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// Rough Plan:
/// Organisation should run in test-mode and listen for commands from the outside.
/// I could have another application that sends commands to it and that way I could test it.
///
/// peerset smart contract prototype:
/// - refactor to only use a single smart contract.
/// - boostrap a single peerset somehow.
///
/// Plan for now:
/// - create a test coordinator that can send commands to the organisation.
/// - create test-mode for organisation that listens for commands.
/// - create a docker-compose that starts the organisation and the coordinator.
/// - run test using the coordinator.
///
#[tokio::main]
async fn main() -> Result<()> {
    shared_init()?;

    let port = std::env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::1]:{}", port).parse()?;
    info!("Running on: {}", addr);

    // todo create a factory that creates the correct client based on the environment.
    let executing_organisation = demo_organisation_one()?;
    let ethereum_client = create_demo_client(executing_organisation)?;

    let organisation_service = OrganisationDevService::new(ethereum_client);

    Server::builder()
        .add_service(OrganisationDevServer::new(organisation_service))
        .serve(addr)
        .await?;

    Ok(())
}

pub struct OrganisationDevService {
    // for communicating with the ethereum blockchain.
    ethereum_client: EnrichedEthereumClient,

    // all peersets that this organisation is part of.
    local_registry: PeersetsLocalRegistry,

    ipfs_client: IPFSClientFacade,
}

/// This struct will be used to record the state of the organisation.
/// mutex should probably be moved to the service?.
/// Maybe we could instead use some sort of actor model?
struct PeersetsLocalRegistry {
    peersets: Arc<Mutex<Vec<Arc<Mutex<PeerSetSmartContractService>>>>>,
}

impl PeersetsLocalRegistry {
    fn new() -> Self {
        PeersetsLocalRegistry {
            peersets: Arc::new(Mutex::new(vec![])),
        }
    }

    // todo: deadcode!!
    #[allow(dead_code)]
    fn find_by_address(
        &self,
        peerset_address: Address,
    ) -> Option<Arc<Mutex<PeerSetSmartContractService>>> {
        self.peersets
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.lock().unwrap().address() == peerset_address)
            .map(|x| Arc::clone(x))
    }

    fn add(&self, peerset_service: PeerSetSmartContractService) {
        let address = peerset_service.address();
        let mut guard = self.peersets.lock().unwrap();

        if guard.iter().any(|x| x.lock().unwrap().address() == address) {
            warn!(
                "Peerset already exists in local registry, duplicate detected: {}",
                address
            );
            return;
        }

        guard.push(Arc::new(Mutex::new(peerset_service)));
    }
}

// todo: refactor to use dependency injection/traits so that we can unit test this!
// todo: this should instead emit events to ProtocolFacade!
impl OrganisationDevService {
    fn new(ethereum_client: EnrichedEthereumClient) -> Self {
        OrganisationDevService {
            ipfs_client: IPFSClientFacade {},
            ethereum_client,
            local_registry: PeersetsLocalRegistry::new(),
        }
    }

    async fn create_peerset_impl(
        &self,
        request: CreatePeersetRequest,
    ) -> Result<CreatePeersetResponse> {
        fn sanitize_peerset_request(request: &CreatePeersetRequest) -> Result<PeerSet> {
            let peers = request
                .peers
                .iter()
                .map(|x| {
                    Ok(Peer {
                        ethereum_address: x.parse()?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(PeerSet { peers })
        }

        // todo: upload graph to ipfs.
        // and register peerset with bootstrapped graph.

        let peer_set = sanitize_peerset_request(&request)?;
        let smart_contract = self
            .ethereum_client
            .register_peerset(&peer_set, demo_graph_ipfs_pointer())
            .await?;

        let address = smart_contract.address().to_string();
        let peerset_service = PeerSetSmartContractService { smart_contract };

        self.local_registry.add(peerset_service);

        Ok(CreatePeersetResponse {
            deployed_peerset_smart_contract_address: address,
        })
    }

    fn peerset_created_impl(
        &self,
        request: PeersetCreatedRequest,
    ) -> Result<PeersetCreatedResponse> {
        let address = &request.deployed_peerset_smart_contract_address;

        let smart_contract = self.ethereum_client.connect_to_peer_set_sc(address)?;

        self.local_registry.add(smart_contract);
        // todo: should also subscribe to peerset smart contract events.

        Ok(PeersetCreatedResponse {})
    }

    async fn propose_change_impl(
        &self,
        request: ProposeChangeRequest,
    ) -> Result<ProposeChangeResponse> {
        let _peerset_address = request.peerset_address;

        let _new_cid = self
            .ipfs_client
            .upload_permission_graph(request.new_permission_graph.unwrap())
            .await?;

        // now we need to propose a change through peerset smart contract.
        // let peerset_service = self.local_registry.find_by_address(peerset_address.into()?)
        //     .ok_or_else(|| eyre!("Peerset not found in local registry: {}", peerset_address))?;
        //
        // // todo: locking should be moved to the service!
        // let _lock = peerset_service.lock()?;
        // _lock.propose_change(new_cid).await?;

        // now we need to wait for the protocol to finish executing
        // and hopefully emit some event that we can listen to.

        panic!();
    }
}

fn handle_err<T>(result: Result<T>) -> std::result::Result<Response<T>, Status> {
    result
        .map(|x| Response::new(x))
        .map_err(|e| Status::internal(e.to_string()))
}

#[tonic::async_trait]
impl OrganisationDev for OrganisationDevService {
    async fn create_peerset(
        &self,
        request: Request<CreatePeersetRequest>,
    ) -> std::result::Result<Response<CreatePeersetResponse>, Status> {
        info!("Creating a peerset: {:?}", request);

        let result = self.create_peerset_impl(request.into_inner()).await;

        handle_err(result)
    }

    async fn peerset_created(
        &self,
        request: Request<PeersetCreatedRequest>,
    ) -> std::result::Result<Response<PeersetCreatedResponse>, Status> {
        info!("Peerset created: {:?}", request);

        let result = self.peerset_created_impl(request.into_inner());

        handle_err(result)
    }

    async fn propose_change(
        &self,
        request: Request<ProposeChangeRequest>,
    ) -> std::result::Result<Response<ProposeChangeResponse>, Status> {
        info!("Proposing a change: {:?}", request);

        todo!()
    }
}

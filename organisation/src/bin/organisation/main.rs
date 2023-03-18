use std::sync::{Arc, Mutex};
use color_eyre::Report;
use ethers::types::Address;
use itertools::Itertools;
use grpc::command::organisation_dev_server::OrganisationDev;
use log::info;
use organisation::data_model::peer_set::{Peer, PeerSet};
use organisation::errors::Result;
use organisation::grpc;
use organisation::grpc::command::organisation_dev_client::OrganisationDevClient;
use organisation::grpc::command::organisation_dev_server::OrganisationDevServer;
use organisation::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse,
};
use organisation::on_chain::ethereum_client::EnrichedEthereumClient;
use organisation::on_chain::peer_broadcast_sc::PeerBroadcastService;
use organisation::poc::shared::{
    create_demo_client, demo_graph_ipfs_pointer,
    demo_organisation_one, demo_peer_set_with_two_peers,
    shared_init,
};
use tonic::transport::{Endpoint, Server};
use tonic::{Request, Response, Status};
use organisation::on_chain::peer_set_sc::PeerSetSmartContractService;

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

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "50051".to_string());
    let addr = format!("[::1]:{}", port).parse()?;
    info!("Running on: {}", addr);

    // todo create a factory that creates the correct client based on the environment.
    let executing_organisation = demo_organisation_one()?;
    let ethereum_client =
        create_demo_client(executing_organisation)?;

    let organisation_service =
        OrganisationDevService::new(ethereum_client);

    Server::builder()
        .add_service(OrganisationDevServer::new(
            organisation_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}

pub struct OrganisationDevService {
    // for communicating with the ethereum blockchain.
    ethereum_client: EnrichedEthereumClient,

    // all peersets that this organisation is part of.
    local_registry: PeersetsLocalRegistry,
}

/// This struct will be used to record the state of the organisation.
struct PeersetsLocalRegistry {
    peersets: Arc<Mutex<Vec<
        Arc<Mutex<PeerSetSmartContractService>>
    >>>
}

impl PeersetsLocalRegistry {
    fn new() -> Self {
        PeersetsLocalRegistry {
            peersets: Arc::new(Mutex::new(vec![])),
        }
    }

    fn find_by_address(&self, peerset_address: Address) -> Option<Arc<Mutex<PeerSet>>> {
        self.peersets
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.address == peerset_address)
            .cloned()
    }

    fn add(&self, peerset_service: PeerSetSmartContractService) {
        self.peersets.lock().unwrap().push(Arc::new(
            Mutex::new(peerset_service))
        );
    }
}

impl OrganisationDevService {
    fn new(ethereum_client: EnrichedEthereumClient) -> Self {
        OrganisationDevService {
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
                .into_iter()
                .map(|x| Ok(Peer {
                    ethereum_address: x.parse()?,
                }))
                .collect::<Result<Vec<_>>>()?;
            Ok(PeerSet { peers })
        }

        let peer_set = sanitize_peerset_request(&request)?;
        let smart_contract = self
            .ethereum_client
            .register_peerset(
                &peer_set,
                demo_graph_ipfs_pointer(),
            )
            .await?;

        let peerset_service = PeerSetSmartContractService {
            smart_contract,
        };

        self.local_registry.add(peerset_service);

        Ok(CreatePeersetResponse {
            deployed_peerset_smart_contract_address: smart_contract.address().to_string(),
        })
    }
}

#[tonic::async_trait]
impl OrganisationDev for OrganisationDevService {
    async fn create_peerset(
        &self,
        request: Request<CreatePeersetRequest>,
    ) -> std::result::Result<
        Response<CreatePeersetResponse>,
        Status,
    > {
        info!("Creating a peerset: {:?}", request);
        // todo graph structure should be part of a request to create a peerset.

        self.create_peerset_impl(request.into_inner())
            .await
            .map(|x| Response::new(x))
            .map_err(|e| Status::internal(e.to_string()))
    }
}

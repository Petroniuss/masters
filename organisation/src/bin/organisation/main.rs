use ethers::types::Address;
use grpc::command::organisation_dev_server::OrganisationDev;

use log::{info, warn};
use organisation::data_model::peer_set::{Peer, PeerSet};
use organisation::errors::Result;
use organisation::grpc;

use ethers_signers::{LocalWallet, Signer};
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
    create_demo_client, demo_graph_ipfs_pointer, demo_organisation_one, shared_init, CHAIN_ID,
};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use organisation::core::protocol::ProtocolFacade;
use organisation::ipfs::ipfs_client::IPFSClientFacade;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

struct Configuration {
    port: String,
    wallet_pk: String,
}

fn load_configuration() -> Configuration {
    let profile = std::env::var("ORG_PROFILE").expect("ORG_PROFILE should be set");

    match profile.as_str() {
        "peer_1" => Configuration {
            port: "50051".to_string(),
            wallet_pk: "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969"
                .to_string(),
        },
        "peer_2" => Configuration {
            port: "50052".to_string(),
            wallet_pk: "d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e"
                .to_string(),
        },
        _ => {
            panic!("Unknown profile {}", profile);
        }
    }
}

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
    let configuration = load_configuration();

    let addr = format!("[::1]:{}", configuration.port).parse()?;
    info!("Running on: {}", addr);

    let wallet = local_wallet(&configuration.wallet_pk);

    let protocol_facade = ProtocolFacade::new(wallet);
    let organisation_service = OrganisationDevService::new(protocol_facade);

    Server::builder()
        .add_service(OrganisationDevServer::new(organisation_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn local_wallet(wallet_address: &str) -> LocalWallet {
    wallet_address
        .parse::<LocalWallet>()
        .expect("should be valid wallet")
        .with_chain_id(CHAIN_ID)
}

pub struct OrganisationDevService {
    protocol_facade: ProtocolFacade,
}

impl OrganisationDevService {
    fn new(protocol_facade: ProtocolFacade) -> Self {
        OrganisationDevService { protocol_facade }
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

        todo!();
    }

    fn peerset_created_impl(
        &self,
        request: PeersetCreatedRequest,
    ) -> Result<PeersetCreatedResponse> {
        todo!();

        Ok(PeersetCreatedResponse {})
    }

    async fn propose_change_impl(
        &self,
        request: ProposeChangeRequest,
    ) -> Result<ProposeChangeResponse> {
        todo!();
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
        todo!();
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

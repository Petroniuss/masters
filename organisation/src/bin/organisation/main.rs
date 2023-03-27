use log::info;
use organisation::core::protocol::ProtocolFacade;
use organisation::errors::Result;
use organisation::shared::shared::shared_init;

use organisation::core::ethereum::local_wallet;
use organisation::transport::grpc::command::organisation_dev_server::{
    OrganisationDev, OrganisationDevServer,
};
use organisation::transport::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse, PeersetCreatedRequest, PeersetCreatedResponse,
    ProposeChangeRequest, ProposeChangeResponse, QueryPeersetsCiDsRequest,
    QueryPeersetsCiDsResponse,
};
use std::fmt::Display;
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

    let organisation_server = OrganisationDevServer::new(organisation_service);

    Server::builder()
        .add_service(organisation_server)
        .serve(addr)
        .await?;

    Ok(())
}

/// grpc generated stubs
pub struct OrganisationDevService {
    protocol_facade: ProtocolFacade,
}

impl OrganisationDevService {
    fn new(protocol_facade: ProtocolFacade) -> Self {
        OrganisationDevService { protocol_facade }
    }
}

#[tonic::async_trait]
impl OrganisationDev for OrganisationDevService {
    async fn create_peerset(
        &self,
        request: Request<CreatePeersetRequest>,
    ) -> std::result::Result<Response<CreatePeersetResponse>, Status> {
        info!("Creating a peerset: {:?}", request);

        let result = self
            .protocol_facade
            .create_peerset(request.into_inner())
            .await;

        handle_err_std(result)
    }

    // ideally this shouldn't exist.
    async fn peerset_created(
        &self,
        request: Request<PeersetCreatedRequest>,
    ) -> std::result::Result<Response<PeersetCreatedResponse>, Status> {
        info!("Peerset created: {:?}", request);

        self.protocol_facade
            .peerset_created(request.into_inner())
            .await;
        let result = Ok(PeersetCreatedResponse {});

        handle_err(result)
    }

    async fn propose_change(
        &self,
        request: Request<ProposeChangeRequest>,
    ) -> std::result::Result<Response<ProposeChangeResponse>, Status> {
        info!("Proposing a change: {:?}", request);

        let result = self
            .protocol_facade
            .propose_change(request.into_inner())
            .await;
        handle_err(Ok(result))
    }

    async fn query_peersets_cid(
        &self,
        request: Request<QueryPeersetsCiDsRequest>,
    ) -> std::result::Result<Response<QueryPeersetsCiDsResponse>, Status> {
        let result = self
            .protocol_facade
            .query_peersets(request.into_inner())
            .await;
        handle_err(Ok(result))
    }
}

/// error handling
fn handle_err<T>(result: Result<T>) -> std::result::Result<Response<T>, Status> {
    result
        .map(|x| Response::new(x))
        .map_err(|e| Status::internal(e.to_string()))
}

fn handle_err_std<T, E: Display>(
    result: std::result::Result<T, E>,
) -> std::result::Result<Response<T>, Status> {
    result
        .map(|x| Response::new(x))
        .map_err(|e| Status::internal(e.to_string()))
}

/// configuration
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

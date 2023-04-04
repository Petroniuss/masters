use crate::core::ethereum::local_wallet;
use crate::core::protocol::ProtocolFacade;
use crate::errors::Result;
use crate::shared::shared::init;
use crate::transport::grpc::command::organisation_dev_server::{
    OrganisationDev, OrganisationDevServer,
};
use crate::transport::grpc::command::{
    CreatePeersetRequest, CreatePeersetResponse, PeersetCreatedRequest, PeersetCreatedResponse,
    ProposeChangeRequest, ProposeChangeResponse, QueryPeersetsCiDsRequest,
    QueryPeersetsCiDsResponse,
};
use log::info;
use std::fmt::Display;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// grpc generated stubs
pub struct OrganisationDevService {
    protocol_facade: ProtocolFacade,
}

impl OrganisationDevService {
    pub(crate) fn new(protocol_facade: ProtocolFacade) -> Self {
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
        let result = PeersetCreatedResponse {};

        handle_err(Ok(result))
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

use grpc::command;
use grpc::command::organisation_dev_client::OrganisationDevClient;
use log::info;
use tonic::transport::Endpoint;

// use organisation::errors::Result;
use organisation::grpc;
use organisation::poc::shared;
use organisation::poc::shared::shared_init;

// use tonic::{transport::Server, Request, Response, Status};

// pub mod grpc {
//     tonic::include_proto!("command"); // The string specified here must match the proto package name
// }

/// **coordinator**
///
/// The coordinator is used for testing purposes,
/// It sends commands via gRPC to nodes to verify their behaviour.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shared_init()?;
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = OrganisationDevClient::new(channel);

    let response = client
        .create_peerset(tonic::Request::new(
            command::CreatePeersetRequest {
                name: "PeerSet-1".to_string(),
                peers: vec![
                    shared::ORGANISATION_ONE_ADDR.to_string(),
                    shared::ORGANISATION_TWO_ADDR.to_string(),
                ],
            },
        ))
        .await?;

    info!("Create peerset response={:?}", response);

    Ok(())
}

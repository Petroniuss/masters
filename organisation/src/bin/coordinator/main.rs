use grpc::command;
use grpc::command::organisation_dev_client::OrganisationDevClient;
use log::info;
use std::collections::HashMap;
use tonic::transport::Endpoint;

use organisation::grpc;
use organisation::grpc::command::{
    Edge, Edges, Node, NodeType, PermissionGraph,
};
use organisation::poc::shared;
use organisation::poc::shared::shared_init;

/// **coordinator**
///
/// The coordinator is used for testing purposes,
/// It sends commands via gRPC to nodes to verify their behaviour.
///
///
/// Scenario 1:
/// Initialise a transaction that spans a single peerset:
/// - create a peerset by peer_1,
/// - notify peer_2 that the peerset has been created,
/// - propose a change by peer_2,
/// - tell peer_1 to acknowledge proposed change,
/// - query both peers about the peerset state to verify that the change has been applied.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shared_init()?;
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client_1 = OrganisationDevClient::new(channel);

    let channel = Endpoint::from_static("http://[::1]:50052")
        .connect()
        .await?;
    let mut client_2 = OrganisationDevClient::new(channel);

    let response = client_1
        .create_peerset(tonic::Request::new(
            command::CreatePeersetRequest {
                name: "PeerSet-1".to_string(),
                peers: vec![
                    shared::ORGANISATION_ONE_ADDR.to_string(),
                    shared::ORGANISATION_TWO_ADDR.to_string(),
                ],
                initial_permission_graph: Some(
                    PermissionGraph {
                        edges: HashMap::from([
                            (
                                "ur_1".to_string(),
                                Edges {
                                    source: Some(Node {
                                        id: "ur_1".to_string(),
                                        r#type: NodeType::User
                                            as i32,
                                        peerset_address: None,
                                    }),
                                    edges: vec![Edge {
                                        destination_node_id:
                                            "ur_2".to_string(),
                                        permission: "belongs"
                                            .to_string(),
                                    }],
                                },
                            ),
                            (
                                "gr_1".to_string(),
                                Edges {
                                    source: Some(Node {
                                        id: "gr_1".to_string(),
                                        r#type: NodeType::User
                                            as i32,
                                        peerset_address: None,
                                    }),
                                    edges: vec![],
                                },
                            ),
                        ]),
                    },
                ),
            },
        ))
        .await?;
    info!("Create peerset response={:?}", response);

    let response = client_2
        .peerset_created(tonic::Request::new(
            command::PeersetCreatedRequest {
                deployed_peerset_smart_contract_address: response.into_inner().deployed_peerset_smart_contract_address,
            },
        )).await?;
    info!("Notify peer2 that peerset PS-1 has been created = response{:?}", response);

    Ok(())
}

use backoff::future::retry;
use backoff::ExponentialBackoff;
use grpc::command;
use grpc::command::organisation_dev_client::OrganisationDevClient;
use log::info;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

use organisation::grpc;
use organisation::grpc::command::{Edge, Edges, Node, NodeType, PermissionGraph};
use organisation::poc::shared;
use organisation::poc::shared::shared_init;

async fn connect(endpoint: &'static str) -> Channel {
    retry(ExponentialBackoff::default(), || async {
        info!("Connecting to node at {}", endpoint);
        let channel = Endpoint::from_static(endpoint).connect().await?;
        Ok(channel)
    })
    .await
    .expect("should be able to connect to node")
}

/// **coordinator**
///
/// The coordinator is used for testing purposes,
/// It sends commands via gRPC to nodes to verify their behaviour & state.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shared_init()?;
    let channel = connect("http://[::1]:50051").await;
    let mut client_1 = OrganisationDevClient::new(channel);

    let channel = connect("http://[::1]:50052").await;
    let mut client_2 = OrganisationDevClient::new(channel);

    info!("Coordinator says hi");
    sleep(Duration::from_secs(20));

    let permission_graph_p1_v1 = PermissionGraph {
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

    let response = client_1
        .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
            name: "p1".to_string(),
            peers: vec![
                shared::ORGANISATION_ONE_ADDR.to_string(),
                shared::ORGANISATION_TWO_ADDR.to_string(),
            ],
            initial_permission_graph: Some(permission_graph_p1_v1.clone()),
        }))
        .await?;
    info!("Create peerset response={:?}", response);
    let peerset_address = response
        .into_inner()
        .deployed_peerset_smart_contract_address;

    // instead we could have a separate smart contract where all peersets are registered.
    let response = client_2
        .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
            deployed_peerset_smart_contract_address: peerset_address.clone(),
        }))
        .await?;
    info!(
        "Notify peer2 that peerset PS-1 has been created = response{:?}",
        response
    );

    // adds a new user that also belongs to the gr_1
    let permission_graph_p1_v2 = {
        let mut tmp = permission_graph_p1_v1.clone();

        tmp.edges.insert(
            "ur_2".to_string(),
            Edges {
                source: Some(Node {
                    id: "ur_2".to_string(),
                    r#type: NodeType::User as i32,
                    peerset_address: None,
                }),
                edges: vec![Edge {
                    destination_node_id: "gr_1".to_string(),
                    permission: "belongs".to_string(),
                }],
            },
        );

        tmp
    };

    // Now propose a change, sync request waits for change to be either accepted or rejected.
    info!("Peer 2 proposes a change to peerset (p1)");
    let response = client_2
        .propose_change(tonic::Request::new(command::ProposeChangeRequest {
            peerset_address,
            new_permission_graph: Some(permission_graph_p1_v2),
        }))
        .await?;
    info!(
        "Reached consensus on proposed change = response{:?}",
        response
    );

    Ok(())
}

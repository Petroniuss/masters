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

    let permission_graph_p1_v1 = shared::demo_graph();

    let peers = vec![
        shared::ORGANISATION_ONE_ADDR.to_string(),
        shared::ORGANISATION_TWO_ADDR.to_string(),
    ];

    let peerset_response = client_1
        .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
            name: "p1".to_string(),
            peers: peers.clone(),
            initial_permission_graph: Some(permission_graph_p1_v1.clone()),
        }))
        .await?
        .into_inner();
    info!("Create peerset response={:?}", peerset_response);

    // todo: this should be handled by blockchain automagically if peerset is registered somewhere.
    // notify peer 1
    let response = client_1
        .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
            deployed_peerset_smart_contract_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peerset_response.cid.clone(),
            peers: peers.clone(),
        }))
        .await?;
    info!(
        "Notify peer1 that peerset PS-1 has been created = response{:?}",
        response
    );

    // notify peer 2
    let response = client_2
        .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
            deployed_peerset_smart_contract_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peerset_response.cid,
            peers: peers.clone(),
        }))
        .await?;
    info!(
        "Notify peer2 that peerset PS-1 has been created = response{:?}",
        response
    );

    // ask them for current graph version.

    // adds a new user that also belongs to the gr_1
    // let permission_graph_p1_v2 = {
    //     let mut tmp = permission_graph_p1_v1.clone();
    //
    //     tmp.edges.insert(
    //         "ur_2".to_string(),
    //         Edges {
    //             source: Some(Node {
    //                 id: "ur_2".to_string(),
    //                 r#type: NodeType::User as i32,
    //                 peerset_address: None,
    //             }),
    //             edges: vec![Edge {
    //                 destination_node_id: "gr_1".to_string(),
    //                 permission: "belongs".to_string(),
    //             }],
    //         },
    //     );
    //
    //     tmp
    // };

    // Now propose a change, sync request waits for change to be either accepted or rejected.
    // info!("Peer 2 proposes a change to peerset (p1)");
    // let response = client_2
    //     .propose_change(tonic::Request::new(command::ProposeChangeRequest {
    //         peerset_address,
    //         new_permission_graph: Some(permission_graph_p1_v2),
    //     }))
    //     .await?;
    // info!(
    //     "Reached consensus on proposed change = response{:?}",
    //     response
    // );

    Ok(())
}

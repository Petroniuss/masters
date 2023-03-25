use backoff::future::retry;
use backoff::ExponentialBackoff;

use log::info;
use organisation::transport::grpc::command;
use tonic::transport::{Channel, Endpoint};

use organisation::poc::shared;
use organisation::poc::shared::shared_init;

use organisation::transport::grpc::command::organisation_dev_client::OrganisationDevClient;
use organisation::transport::grpc::command::{
    Edge, Edges, Node, NodeType, PeersetGraph, QueryPeersetsCiDsRequest,
};

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

    info!("Creating peerset..");
    let peerset_response = client_1
        .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
            name: "p1".to_string(),
            peers: peers.clone(),
            initial_permission_graph: Some(permission_graph_p1_v1.clone()),
        }))
        .await?
        .into_inner();
    info!("Peerset Created {:?}", peerset_response);

    info!("Notifying peer2 that peerset PS-1 has been created..");
    let _response = client_2
        .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
            deployed_peerset_smart_contract_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peerset_response.cid.clone(),
            peers: peers.clone(),
        }))
        .await?;

    info!("Querying peer1 to get their perceived version of the graph..");
    let response = client_1
        .query_peersets_cid(QueryPeersetsCiDsRequest {})
        .await?
        .into_inner();
    info!("Peer1 response: {:?}", response);
    assert_eq!(response.peerset_graphs.len(), 1);
    assert_eq!(
        response.peerset_graphs[0],
        PeersetGraph {
            peerset_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peerset_response.cid.clone(),
        }
    );

    info!("Querying peer2 to get their perceived version of the graph..");
    let response = client_2
        .query_peersets_cid(QueryPeersetsCiDsRequest {})
        .await?
        .into_inner();
    info!("Peer2 response: {:?}", response);

    assert_eq!(response.peerset_graphs.len(), 1);
    assert_eq!(
        response.peerset_graphs[0],
        PeersetGraph {
            peerset_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peerset_response.cid.clone(),
        }
    );

    // ask them for current graph version.

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

    info!("Proposing a change by peer 1..");
    let response = client_1
        .propose_change(tonic::Request::new(command::ProposeChangeRequest {
            peerset_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            new_permission_graph: Some(permission_graph_p1_v2),
        }))
        .await?;
    info!(
        "Reached consensus on proposed change = response{:?}",
        response
    );

    Ok(())
}

use log::info;
use organisation::core::grpc::connect;
use organisation::shared::shared;
use organisation::transport::grpc::command;
use organisation::transport::grpc::command::organisation_dev_client::OrganisationDevClient;
use organisation::transport::grpc::command::{
    Edge, Edges, Node, NodeType, PeersetGraph, QueryPeersetsCiDsRequest,
};

/// this has been largely replaced by integration tests.
/// **coordinator**
///
/// The coordinator is used for testing purposes,
/// It sends commands via gRPC to nodes to verify their behaviour & state.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shared::init()?;
    let channel = connect("http://[::1]:50051").await;
    let mut client_1 = OrganisationDevClient::new(channel);

    let channel = connect("http://[::1]:50052").await;
    let mut client_2 = OrganisationDevClient::new(channel);

    let permission_graph_p1_v1 = shared::demo_graph_p1_v1();

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

    info!("Proposing a change by peer 2..");
    let peer_2_voting_response = client_2
        .propose_change(tonic::Request::new(command::ProposeChangeRequest {
            peerset_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            new_permission_graph: Some(permission_graph_p1_v2),
        }))
        .await?
        .into_inner();
    info!(
        "Peer 2 reports that voting has been completed = response{:?}",
        peer_2_voting_response
    );

    info!("Querying peer1 to get their perceived version of the graph..");
    let response = client_1
        .query_peersets_cid(QueryPeersetsCiDsRequest {})
        .await?
        .into_inner();

    // todo: this is a race condition
    // eventually an assertion should pass with some timeout.
    info!("Peer1 response: {:?}", response);
    assert_eq!(response.peerset_graphs.len(), 1);
    assert_eq!(
        response.peerset_graphs[0],
        PeersetGraph {
            peerset_address: peerset_response
                .deployed_peerset_smart_contract_address
                .clone(),
            permission_graph_cid: peer_2_voting_response.proposed_cid.clone(),
        }
    );

    Ok(())
}

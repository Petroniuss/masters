use log::info;
use organisation::core::ethereum::AddressToString;
use organisation::core::grpc::connect;
use organisation::core::peer::{
    peer_1_configuration, peer_2_configuration, peer_3_configuration, peer_4_configuration,
    run_with_configuration,
};
use organisation::errors::Result;
use organisation::shared::shared;
use organisation::shared::shared::init;
use organisation::transport::grpc::command;
use organisation::transport::grpc::command::organisation_dev_client::OrganisationDevClient;
use organisation::transport::grpc::command::{
    Edge, Edges, Node, NodeType, PeersetGraph, QueryPeersetsCiDsRequest,
};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

/// Integration test verifying change within a single peerset:
/// - spawn two peers in a single peerset,
/// - create peerset_1,
/// - propose a change,
/// - approve changes in both peersets,
/// - verify that both peers see the same state
#[tokio::test]
async fn within_peerset_change() -> Result<()> {
    init()?;

    info!("Spawning peers...");
    let peer_1_conf = peer_1_configuration();
    {
        let conf = peer_1_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    let peer_2_conf = peer_2_configuration();
    {
        let conf = peer_2_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    info!("Creating peerset 1...");
    let channel = connect(peer_1_conf.local_connection_str().as_str()).await;
    let mut client_1 = OrganisationDevClient::new(channel);

    let channel = connect(peer_2_conf.local_connection_str().as_str()).await;
    let mut client_2 = OrganisationDevClient::new(channel);
    let peerset_address = {
        let permission_graph_p1_v1 = shared::demo_graph_p1_v1();
        let peers = vec![
            peer_1_conf.address().to_full_string(),
            peer_2_conf.address().to_full_string(),
        ];

        let peerset_response = client_1
            .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
                name: "p1".to_string(),
                peers: peers.clone(),
                initial_permission_graph: Some(permission_graph_p1_v1.clone()),
            }))
            .await?
            .into_inner();
        info!("Created Peerset 1: {:?}", peerset_response);

        info!("Notifying peer2 that peerset 1 has been created..");
        let _response = client_2
            .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
                deployed_peerset_smart_contract_address: peerset_response
                    .deployed_peerset_smart_contract_address
                    .clone(),
                permission_graph_cid: peerset_response.cid.clone(),
                peers: peers.clone(),
            }))
            .await?;

        peerset_response.deployed_peerset_smart_contract_address
    };

    info!("Proposing a change by peer 2..");
    let peer_2_voting_response = {
        let permission_graph_p1_v2 = shared::demo_graph_p1_v2();

        client_2
            .propose_change(tonic::Request::new(command::ProposeChangeRequest {
                peerset_address,
                new_permission_graph: Some(permission_graph_p1_v2),
            }))
            .await?
            .into_inner()
    };

    info!(
        "Peer 2 reports that voting has been completed = response{:?}",
        peer_2_voting_response
    );

    info!("Querying peer1 to get their perceived version of the graph..");
    let response = client_1
        .query_peersets_cid(QueryPeersetsCiDsRequest {})
        .await?
        .into_inner();

    info!("Waiting for cross-peerset change to be acknowledged in peerset 2..");
    eventually_passes(
        || async {
            client_1
                .clone()
                .query_peersets_cid(tonic::Request::new(QueryPeersetsCiDsRequest {}))
                .await
                .unwrap()
                .into_inner()
        },
        |response| {
            response.peerset_graphs[0].permission_graph_cid
                == peer_2_voting_response.proposed_cid.clone()
        },
        "current cid should be set by cross-peerset change",
    )
    .await;

    Ok(())
}

/// Integration test verifying atomic commitment:
/// - spawn two peers in each peerset,
/// - create peerset_1,
/// - create peerset_2,
/// - subscribe for cross_peerset_changes,
/// - propose a cross_peerset_change,
/// - approve changes in both peersets,
#[tokio::test]
async fn cross_peerset_change() -> Result<()> {
    init()?;

    info!("Spawning peers...");
    let peer_1_conf = peer_1_configuration();
    {
        let conf = peer_1_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    let peer_2_conf = peer_2_configuration();
    {
        let conf = peer_2_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    let peer_3_conf = peer_3_configuration();
    {
        let conf = peer_3_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    let peer_4_conf = peer_4_configuration();
    {
        let conf = peer_4_conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    info!("Creating peerset 1...");
    let channel = connect(peer_1_conf.local_connection_str().as_str()).await;
    let mut client_1 = OrganisationDevClient::new(channel);

    let channel = connect(peer_2_conf.local_connection_str().as_str()).await;
    let mut client_2 = OrganisationDevClient::new(channel);
    let peerset_1_address = {
        let permission_graph_p1_v1 = shared::demo_graph_p1_v1();
        let peers = vec![
            peer_1_conf.address().to_full_string(),
            peer_2_conf.address().to_full_string(),
        ];

        let peerset_response = client_1
            .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
                name: "p1".to_string(),
                peers: peers.clone(),
                initial_permission_graph: Some(permission_graph_p1_v1.clone()),
            }))
            .await?
            .into_inner();
        info!("Created Peerset 1: {:?}", peerset_response);

        info!("Notifying peer2 that peerset 1 has been created..");
        let _response = client_2
            .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
                deployed_peerset_smart_contract_address: peerset_response
                    .deployed_peerset_smart_contract_address
                    .clone(),
                permission_graph_cid: peerset_response.cid.clone(),
                peers: peers.clone(),
            }))
            .await?;

        peerset_response.deployed_peerset_smart_contract_address
    };

    info!("Creating peerset 2...");
    let channel = connect(peer_3_conf.local_connection_str().as_str()).await;
    let mut client_3 = OrganisationDevClient::new(channel);

    let channel = connect(peer_4_conf.local_connection_str().as_str()).await;
    let mut client_4 = OrganisationDevClient::new(channel);
    let peerset_2_address = {
        let permission_graph_p1_v1 = shared::demo_graph_p1_v1();
        let peers = vec![
            peer_3_conf.address().to_full_string(),
            peer_4_conf.address().to_full_string(),
        ];

        let peerset_response = client_3
            .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
                name: "p2".to_string(),
                peers: peers.clone(),
                initial_permission_graph: Some(permission_graph_p1_v1.clone()),
            }))
            .await?
            .into_inner();
        info!("Created Peerset 2: {:?}", peerset_response);

        info!("Notifying peer4 that peerset 2 has been created..");
        let _response = client_4
            .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
                deployed_peerset_smart_contract_address: peerset_response
                    .deployed_peerset_smart_contract_address
                    .clone(),
                permission_graph_cid: peerset_response.cid.clone(),
                peers: peers.clone(),
            }))
            .await?;

        peerset_response.deployed_peerset_smart_contract_address
    };

    info!("Proposing a cross-peerset change");
    let cross_peerset_change_response = {
        let new_permission_graph_ps_1 =
            shared::demo_graph_p1_cross_peerset_v2(peerset_2_address.clone());
        let new_permission_graph_ps_2 =
            shared::demo_graph_p2_cross_peerset_v2(peerset_1_address.clone());

        client_1
            .propose_cross_peerset_change(tonic::Request::new(
                command::ProposeCrossPeersetChangeRequest {
                    peerset_address: peerset_1_address,
                    new_permission_graph: Some(new_permission_graph_ps_1),
                    other_peerset_address: peerset_2_address,
                    other_permission_graph: Some(new_permission_graph_ps_2),
                },
            ))
            .await?
            .into_inner()
    };
    info!(
        "Successfully proposed cross-peerset change: {:?}",
        cross_peerset_change_response
    );

    info!("Waiting for cross-peerset change to be acknowledged in peerset 1..");
    eventually_passes(
        || async {
            client_1
                .clone()
                .query_peersets_cid(tonic::Request::new(QueryPeersetsCiDsRequest {}))
                .await
                .unwrap()
                .into_inner()
        },
        |response| {
            response.peerset_graphs[0].permission_graph_cid
                == cross_peerset_change_response.proposed_cid
        },
        "current cid should be set by cross-peerset change",
    )
    .await;

    info!("Waiting for cross-peerset change to be acknowledged in peerset 2..");
    eventually_passes(
        || async {
            client_4
                .clone()
                .query_peersets_cid(tonic::Request::new(QueryPeersetsCiDsRequest {}))
                .await
                .unwrap()
                .into_inner()
        },
        |response| {
            response.peerset_graphs[0].permission_graph_cid
                == cross_peerset_change_response.other_proposed_cid
        },
        "current cid should be set by cross-peerset change",
    )
    .await;

    Ok(())
}

pub async fn eventually_passes<I, Fun, Fut, P>(
    mut operation: Fun,
    predicate: P,
    expected: &'static str,
) where
    Fun: FnMut() -> Fut,
    P: Fn(I) -> bool,
    Fut: Future<Output = I>,
{
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for _i in 0..10 {
        interval.tick().await;
        let result = operation().await;
        if predicate(result) {
            return;
        }
    }

    panic!("{}", expected)
}

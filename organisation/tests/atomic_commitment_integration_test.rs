use log::info;
use organisation::core::ethereum::AddressToString;
use organisation::core::grpc::connect;
use organisation::core::peer;
use organisation::core::peer::{
    peer_1_configuration, peer_2_configuration, peer_3_configuration, peer_4_configuration,
    run_with_configuration,
};
use organisation::errors::Result;
use organisation::shared::shared;
use organisation::shared::shared::init;
use organisation::transport::grpc::command;
use organisation::transport::grpc::command::organisation_dev_client::OrganisationDevClient;
use std::time::Duration;
use tokio::time::sleep;

/// Integration test verifying atomic commitment:
/// - spawn two peers in each peerset,
/// - create peerset_1,
/// - create peerset_2,
/// - subscribe for cross_peerset_changes,
/// - propose a cross_peerset_change,
/// - approve changes in both peersets,
/// - this could simply be an integration test now that I think about it
#[tokio::test]
async fn atomic_commitment() -> Result<()> {
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

    {
        let permission_graph_p1_v1 = shared::demo_graph();
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
    }

    // create peerset 2
    sleep(Duration::new(2, 0)).await;

    Ok(())
}

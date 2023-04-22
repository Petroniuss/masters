use futures::stream::iter;
use itertools::Itertools;
use log::info;
use organisation::core::ethereum::AddressToString;
use organisation::core::grpc::connect;
use organisation::core::peer::{peers_configurations, run_with_configuration, Configuration};
use organisation::errors::Result;
use organisation::shared::shared;
use organisation::shared::shared::init;
use organisation::transport::grpc::command;
use organisation::transport::grpc::command::organisation_dev_client::OrganisationDevClient;
use organisation::transport::grpc::command::{Edge, Edges, Node, NodeType, PermissionGraph};
use std::time::{Duration, SystemTime};
use tokio::time::sleep;
use tonic::transport::Channel;

#[tokio::test]
async fn cross_peerset_benchmark() -> Result<()> {
    init()?;
    let peers_num = std::env::var("PEERS_NUM")
        .unwrap_or("2".to_string())
        .parse::<usize>()?;

    let iterations = std::env::var("ITER_NUM")
        .unwrap_or("1".to_string())
        .parse::<usize>()?;

    benchmark_iter(peers_num, iterations).await?;

    Ok(())
}

async fn benchmark_iter(peers_num: usize, changes_num: usize) -> Result<()> {
    let mut peers = prepare_peers(peers_num).await?;
    let (mut peers_1, mut peers_2) = peers.split_at_mut(peers_num / 2);

    let peerset_1_address = create_peerset(peers_1).await?;
    let peerset_2_address = create_peerset(peers_2).await?;

    let start = SystemTime::now();
    let mut new_permission_graph_ps_1 = shared::demo_graph_p1_v1();
    let mut new_permission_graph_ps_2 = shared::demo_graph_p2_v1();
    for _ in 0..changes_num {
        let new_permission_graph_ps_1 = add_random_user_to_group(&new_permission_graph_ps_1);
        let new_permission_graph_ps_2 = add_random_user_to_group(&new_permission_graph_ps_2);
        propose_transaction(
            &mut peers,
            &peerset_1_address,
            &peerset_2_address,
            new_permission_graph_ps_1.clone(),
            new_permission_graph_ps_2.clone(),
        )
        .await?;
    }

    let done = SystemTime::now();
    info!("Time elapsed: {:?}", done.duration_since(start)?);
    Ok(())
}

async fn propose_transaction(
    peers: &mut Vec<BenchmarkedPeer>,
    peerset_1_address: &str,
    peerset_2_address: &str,
    new_permission_graph_ps_1: PermissionGraph,
    new_permission_graph_ps_2: PermissionGraph,
) -> Result<()> {
    let client_proposing_change = &mut peers[0].client;

    client_proposing_change
        .propose_cross_peerset_change(tonic::Request::new(
            command::ProposeCrossPeersetChangeRequest {
                peerset_address: peerset_1_address.to_string(),
                new_permission_graph: Some(new_permission_graph_ps_1),
                other_peerset_address: peerset_2_address.to_string(),
                other_permission_graph: Some(new_permission_graph_ps_2),
            },
        ))
        .await?
        .into_inner();

    Ok(())
}

fn add_random_user_to_group(prev_permission_graph: &PermissionGraph) -> PermissionGraph {
    let mut graph = prev_permission_graph.clone();

    loop {
        let rand = rand::random::<usize>() % (10e6 as usize);
        let user_id = format!("ps_1_ur_{}", rand);
        if graph.edges.contains_key(&user_id) {
            continue;
        }

        graph.edges.insert(
            user_id.clone(),
            Edges {
                source: Some(Node {
                    id: user_id.clone(),
                    r#type: NodeType::User as i32,
                    peerset_address: None,
                }),
                edges: vec![Edge {
                    destination_node_id: "ps_1_gr_1".to_string(),
                    permission: "belongs".to_string(),
                }],
            },
        );

        break;
    }

    graph
}

struct BenchmarkedPeer {
    conf: Configuration,
    client: OrganisationDevClient<Channel>,
}

async fn create_peersets(
    peers_1: &mut Vec<BenchmarkedPeer>,
    peers_2: &mut Vec<BenchmarkedPeer>,
) -> Result<(String, String)> {
    let peerset_1 = create_peerset(peers_1).await?;
    let peerset_2 = create_peerset(peers_2).await?;

    Ok((peerset_1, peerset_2))
}

async fn create_peerset(peers: &mut [BenchmarkedPeer]) -> Result<String> {
    let start = SystemTime::now();
    let permission_graph_p1_v1 = shared::demo_graph_p1_v1();
    let peers_addresses = peers
        .iter()
        .map(|peer| peer.conf.address().to_full_string())
        .collect_vec();

    info!("Creating peerset..");
    let peerset_response = peers[0]
        .client
        .create_peerset(tonic::Request::new(command::CreatePeersetRequest {
            name: "p1".to_string(),
            peers: peers_addresses.clone(),
            initial_permission_graph: Some(permission_graph_p1_v1.clone()),
        }))
        .await?
        .into_inner();
    let done = SystemTime::now();
    info!("Peerset created, time: {:?}", done.duration_since(start)?);
    info!("Created Peerset: {:?}", peerset_response);

    info!("Notifying other peers about created peerset..");
    for peer in peers.iter_mut().skip(1) {
        peer.client
            .peerset_created(tonic::Request::new(command::PeersetCreatedRequest {
                deployed_peerset_smart_contract_address: peerset_response
                    .deployed_peerset_smart_contract_address
                    .clone(),
                permission_graph_cid: peerset_response.cid.clone(),
                peers: peers_addresses.clone(),
            }))
            .await?;
    }

    Ok(peerset_response.deployed_peerset_smart_contract_address)
}

async fn prepare_peers(peers_num: usize) -> Result<Vec<BenchmarkedPeer>> {
    // start peers
    let confs = peers_configurations(peers_num);
    for conf in &confs {
        let conf = conf.clone();
        tokio::spawn(async move { run_with_configuration(conf).await });
    }

    // wait until peers are ready
    let mut peers = Vec::new();
    for conf in &confs {
        let channel = connect(conf.local_connection_str().as_str()).await;
        let client = OrganisationDevClient::new(channel);

        peers.push(BenchmarkedPeer {
            conf: conf.clone(),
            client,
        });
    }

    Ok(peers)
}

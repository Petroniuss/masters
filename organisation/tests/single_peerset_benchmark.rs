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
use std::time::SystemTime;
use tonic::transport::Channel;

/// This is a test used to generate results indicating throughput parametrized by number of peers:
/// - how long it takes for a transaction to succeed,
/// todo: let's try using criterion for this...
#[tokio::test]
async fn single_peerset_benchmark() -> Result<()> {
    init()?;

    let mut peers = prepare_peers(3).await?;
    let peerset_address = create_peerset(&mut peers).await?;

    // let's just measure how long it takes for 5 iterations
    let start = SystemTime::now();
    let mut graph = shared::demo_graph_p1_v1();
    for _ in 0..5 {
        graph = add_random_user_to_group(&graph);
        propose_transaction(&mut peers, &peerset_address, &graph).await?;
    }

    let done = SystemTime::now();
    info!("Time elapsed: {:?}", done.duration_since(start)?);
    Ok(())
}

async fn propose_transaction(
    peers: &mut Vec<BenchmarkedPeer>,
    peerset_address: &str,
    permission_graph: &PermissionGraph,
) -> Result<()> {
    let client_proposing_change = &mut peers[0].client;

    client_proposing_change
        .propose_change(tonic::Request::new(command::ProposeChangeRequest {
            peerset_address: peerset_address.to_string(),
            new_permission_graph: Some(permission_graph.clone()),
        }))
        .await?;

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

async fn create_peerset(peers: &mut Vec<BenchmarkedPeer>) -> Result<String> {
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

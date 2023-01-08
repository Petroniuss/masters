use ethers::abi::Address;
use log::info;
use organisation::errors::Result;
use organisation::on_chain::peer_broadcast_sc::PeerBroadcastService;
use organisation::poc::shared::{
    create_demo_client, demo_graph_ipfs_pointer,
    demo_organisation_one, demo_peer_ipfs_pointer,
    demo_peer_set_with_two_peers, shared_init,
    DEMO_PEER_SET_SMART_CONTRACT_ADDRESS,
};
use std::str::FromStr;

/// **register-peer-set**
///
/// This demonstrates how to:
/// - broadcast organisation via peer-broadcast smart contract.
/// - register a peer-set smart contract.
///
/// It assumes that that peer-register smart contract
/// and oracle smart contract have already been deployed.
#[tokio::main]
async fn main() -> Result<()> {
    shared_init()?;

    let executing_organisation = demo_organisation_one()?;
    let ethereum_client =
        create_demo_client(executing_organisation.clone())?;

    info!(
        "Organisation {:?} broadcasts itself as an organisation",
        executing_organisation
    );
    let _ = ethereum_client
        .register_itself(demo_peer_ipfs_pointer())
        .await?;

    let peer_set = demo_peer_set_with_two_peers()?;
    info!(
        "Organisation {:?} is registering a peer-set: {:?}",
        executing_organisation, peer_set
    );
    let _peer_set_smart_contract = ethereum_client
        .register_peerset(&peer_set, demo_graph_ipfs_pointer())
        .await?;
    assert_eq!(
        Address::from_str(
            DEMO_PEER_SET_SMART_CONTRACT_ADDRESS
        )?,
        _peer_set_smart_contract.address()
    );

    Ok(())
}

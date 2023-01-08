use log::info;

use organisation::errors::Result;

use organisation::on_chain::peer_set_sc::PeerSetSmartContractServiceFromAddress;
use organisation::poc::shared::{
    create_demo_client, demo_organisation_two,
    demo_peer_set_with_two_peers, demo_updated_ipfs_pointer,
    shared_init, DEMO_PEER_SET_SMART_CONTRACT_ADDRESS,
};

/// **propose-change**
///
/// This demonstrates how to:
/// - propose a change to a peer-set smart contract.
#[tokio::main]
async fn main() -> Result<()> {
    shared_init()?;
    let executing_organisation = demo_organisation_two()?;
    let ethereum_client =
        create_demo_client(executing_organisation.clone())?;

    let peer_set = demo_peer_set_with_two_peers()?;
    let peer_set_smart_contract = ethereum_client
        .connect_to_peer_set_sc(
            DEMO_PEER_SET_SMART_CONTRACT_ADDRESS,
        )?;

    let changed_ipfs = demo_updated_ipfs_pointer();
    info!(
        "Organisation {:?} is proposing a change: {} to peer-set: {:?}",
        executing_organisation,
        changed_ipfs,
        peer_set
    );
    peer_set_smart_contract
        .propose_change(changed_ipfs)
        .await?;

    Ok(())
}

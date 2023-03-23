use log::info;
use organisation::errors::Result;
use organisation::on_chain::oracle_sc::OracleServiceFromAddress;
use organisation::on_chain::peer_set_sc::PeerSetSmartContractServiceFromAddress;
use organisation::poc::shared::{
    create_demo_client, demo_organisation_one, demo_peer_set_with_two_peers, shared_init,
    DEMO_PEER_SET_SMART_CONTRACT_ADDRESS,
};

/// **validate-change**
///
/// This demonstrates how to:
/// - listen to events from the oracle smart contract
/// - listen to events from the peer set smart contract
/// - submit a change to the oracle smart contract
#[tokio::main]
async fn main() -> Result<()> {
    shared_init()?;
    let executing_organisation = demo_organisation_one()?;
    let ethereum_client = create_demo_client(executing_organisation.clone())?;

    let peer_set = demo_peer_set_with_two_peers()?;
    let peer_set_smart_contract =
        ethereum_client.connect_to_peer_set_sc(DEMO_PEER_SET_SMART_CONTRACT_ADDRESS)?;
    let oracle_sc = ethereum_client.connect_to_oracle()?;
    info!(
        "Organisation {:?} connected to oracle smart contract: {} and peer_set_smart_contract: {}",
        executing_organisation,
        oracle_sc.address(),
        peer_set_smart_contract.address()
    );

    info!(
        "Listening to past events from peer set smart contract: {:?}",
        peer_set_smart_contract.address()
    );
    peer_set_smart_contract.log_past_events().await?;

    info!(
        "Listening for past validation request events from oracle: {:?}",
        oracle_sc.address()
    );
    let event = oracle_sc
        .find_latest_oracle_validation_request(peer_set_smart_contract.address())
        .await?
        .expect("There should be a validation request at this point");

    info!(
        "Organisation {:?} is validating a change to peer-set: {:?} \
        with request-id: {} and given change: {:?}",
        executing_organisation,
        peer_set,
        String::from_utf8_lossy(&event.request_id),
        event.proposed_graph_ipfs_pointer
    );
    oracle_sc.validate_change(event.request_id, true).await?;

    info!(
        "Listening for past validation request events from oracle: {:?}",
        oracle_sc.address()
    );
    peer_set_smart_contract.log_past_events().await?;

    Ok(())
}

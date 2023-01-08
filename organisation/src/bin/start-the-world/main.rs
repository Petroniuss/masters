use ethers::types::Address;
use log::info;
use std::str::FromStr;

use organisation::errors::Result;

use organisation::on_chain::contract_deployment::{
    OracleSmartContractDeployment,
    PeerBroadcastSmartContractDeployment,
};

use organisation::poc::shared::{
    create_base_demo_client, demo_organisation_one,
    shared_init, ORACLE_CONTRACT_ADDRESS,
    PEER_BROADCAST_CONTRACT_ADDRESS,
};

/// **start-the-world**
///
/// deploys oracle and peer-broadcast smart contracts to local blockchain node.
/// this is a one-time operation, other contracts then use these contracts.
#[tokio::main]
async fn main() -> Result<()> {
    shared_init()?;

    let executing_organisation = demo_organisation_one()?;
    info!(
        "Organisation {:?} is starting the world",
        executing_organisation
    );

    let ethereum_client = create_base_demo_client(
        executing_organisation.clone(),
    )?;

    // deploy peer broadcast smart contract
    let _peer_broadcast_sc = ethereum_client
        .deploy_peer_broadcast_smart_contract()
        .await?;
    info!(
        "Deployed peer broadcast smart contract to address {}",
        _peer_broadcast_sc.address()
    );
    assert_eq!(
        Address::from_str(PEER_BROADCAST_CONTRACT_ADDRESS)?,
        _peer_broadcast_sc.address()
    );

    // deploy oracle smart contract
    let _permission_verifier_oracle = ethereum_client
        .deploy_permission_verifier_oracle()
        .await?;
    info!(
        "Deployed oracle smart contract to address {:?}",
        _permission_verifier_oracle.address()
    );
    assert_eq!(
        Address::from_str(ORACLE_CONTRACT_ADDRESS)?,
        _permission_verifier_oracle.address()
    );

    Ok(())
}

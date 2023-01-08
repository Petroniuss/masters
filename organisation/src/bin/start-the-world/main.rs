use ethers_signers::{LocalWallet, Signer};
use log::info;
use organisation::data_model::organisation::{
    ExecutingOrganisation, Organisation,
};

use organisation::errors::Result;

use organisation::on_chain::contract_deployment::{
    OracleSmartContractDeployment,
    PeerBroadcastSmartContractDeployment,
};
use organisation::on_chain::ethereum_client;

use std::sync::Arc;

// todo: this should be configurable (and taken from env variables or sth)
fn executing_organisation() -> Result<Arc<ExecutingOrganisation>>
{
    let wallet =
        "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969"
            .parse::<LocalWallet>()?
            .with_chain_id(31337u64);

    Ok(Arc::new(ExecutingOrganisation {
        organisation: Organisation {
            name: "start-the-world-og".to_string(),
            ethereum_address: wallet.address(),
        },
        wallet,
    }))
}

/// **start-the-world**
///
/// deploys oracle and peer-broadcast smart contracts to local blockchain node.
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();
    let executing_organisation = executing_organisation()?;
    info!(
        "Organisation {} is starting the world",
        executing_organisation.address()
    );

    let ethereum_client =
        ethereum_client::crate_local_ethereum_client(
            executing_organisation.clone(),
        )?;

    let _permission_verifier_oracle = ethereum_client
        .deploy_permission_verifier_oracle()
        .await?;

    let _peer_broadcast_sc = ethereum_client
        .deploy_peer_broadcast_smart_contract()
        .await?;

    Ok(())
}

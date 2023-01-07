extern crate organisation;

use crate::organisation::data_model::peer_set::{
    Peer, PeerSet,
};
use crate::organisation::errors::Result;
use crate::organisation::on_chain::contract_deployment::PeerSetSmartContractDeployment;
use crate::organisation::on_chain::{
    contract_deployment, ethereum_client,
};
use ethers::abi::Address;
use ethers_signers::{LocalWallet, Signer};
use log::info;
use organisation::data_model::organisation::{
    ExecutingOrganisation, Organisation,
};
use organisation::on_chain::contract_deployment::OracleSmartContractDeployment;
use std::str::FromStr;
use std::sync::Arc;

fn example_peer_set_with_two_peers() -> Result<PeerSet> {
    Ok(PeerSet {
        peers: vec![
            Peer {
                ethereum_address: Address::from_str(
                    "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b",
                )?,
            },
            Peer {
                ethereum_address: Address::from_str(
                    "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e",
                )?,
            },
        ],
    })
}

fn executing_organisation() -> Result<Arc<ExecutingOrganisation>>
{
    let wallet =
        "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969"
            .parse::<LocalWallet>()?
            .with_chain_id(31337u64);

    Ok(Arc::new(ExecutingOrganisation {
        organisation: Organisation {
            name: "ORG_A".to_string(),
            ethereum_address: wallet.address(),
        },
        wallet,
    }))
}

fn peer_set_graph_ipfs_pointer() -> String {
    // some random pointer for now.
    return "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"
        .to_string();
}

/// This should demonstrate how to boostrap a couple of peer-sets on blockchain.
/// Ideally the way it should work - we should subscribe to the events from the blockchain
/// and build our local state based on the events/transactions on the ledger.
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    info!("Starting the world");
    let executing_organisation = executing_organisation()?;
    let peer_set = example_peer_set_with_two_peers()?;

    let ethereum_client =
        ethereum_client::crate_local_ethereum_client(
            executing_organisation.clone(),
        )?;

    let contract_deployment_service =
        contract_deployment::SmartContractDeploymentService {
            executing_organisation: executing_organisation
                .clone(),
            ethereum_client: ethereum_client.clone(),
        };

    let permission_verifier_oracle =
        contract_deployment_service
            .deploy_permission_verifier_oracle()
            .await?;

    let _peer_set_smart_contract = contract_deployment_service
        .deploy_peer_set_smart_contract(
            &peer_set,
            &permission_verifier_oracle,
            peer_set_graph_ipfs_pointer(),
        )
        .await?;

    Ok(())
}

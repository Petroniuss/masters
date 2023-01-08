use ethers::abi::Address;
use ethers_signers::{LocalWallet, Signer};
use log::info;

use organisation::data_model::organisation::{
    ExecutingOrganisation, Organisation,
};
use organisation::data_model::peer_set::{Peer, PeerSet};
use organisation::errors::Result;

use organisation::on_chain::ethereum_client::ToEthereumClientEnriched;

use organisation::on_chain::ethereum_client;

use organisation::on_chain::peer_set_sc::PeerSetSmartContractServiceFromAddress;
use std::str::FromStr;
use std::sync::Arc;

// todo: remove duplicate code by moving it into lib.

static ORACLE_CONTRACT_ADDRESS: &'static str =
    "0xbf5a1966ed793a7ca90878701e410463836bb366";
static PEER_BROADCAST_CONTRACT_ADDRESS: &'static str =
    "0x19800ab132174a00e2ab1434678bbc34554cb915";

// todo: this shouldn't be hardcoded - this should be taken from the blockchain
// by querying peer broadcast smart contract.
static PEER_SET_SMART_CONTRACT_ADDRESS: &'static str =
    "0xc0d18d2a4129fec8095a1eb19ef14cc88200a4ac";

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
        "d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e"
            .parse::<LocalWallet>()?
            .with_chain_id(31337u64);

    Ok(Arc::new(ExecutingOrganisation {
        organisation: Organisation {
            name: "peer-1".to_string(),
            ethereum_address: wallet.address(),
        },
        wallet,
    }))
}

fn changed_ipfs_pointer() -> String {
    return "https://ipfs.io/ipfs/updated".to_string();
}

/// This demonstrates how to:
/// - deploy an oracle
/// - boostrap a peer-set smart contract
/// todo: it should also deploy an initial peer_set graph to IPFS.
/// todo: this should assume that the oracle and peer-broadcast sc are already deployed.
#[tokio::main]
async fn main() -> Result<()> {
    // boilerplate
    color_eyre::install()?;
    pretty_env_logger::init();

    let executing_organisation = executing_organisation()?;
    let peer_set = example_peer_set_with_two_peers()?;
    info!(
        "Organisation {} is proposing a change to peer-set: {:?}",
        executing_organisation.address(),
        peer_set
    );

    let ethereum_client =
        ethereum_client::crate_local_ethereum_client(
            executing_organisation.clone(),
        )?;

    let ethereum_client = Arc::new(ethereum_client);

    let enriched_client = ethereum_client.to_enriched_client(
        PEER_BROADCAST_CONTRACT_ADDRESS,
        ORACLE_CONTRACT_ADDRESS,
    )?;

    let peer_set_smart_contract = enriched_client
        .connect_to_peer_set_sc(
            PEER_SET_SMART_CONTRACT_ADDRESS,
        )?;

    peer_set_smart_contract
        .propose_change(changed_ipfs_pointer())
        .await?;

    Ok(())
}

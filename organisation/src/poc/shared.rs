use crate::data_model::organisation::{
    ExecutingOrganisation, Organisation,
};
use crate::data_model::peer_set::{Peer, PeerSet};
use crate::errors::Result;
use crate::on_chain::ethereum_client;
use crate::on_chain::ethereum_client::{
    EnrichedEthereumClient, EthereumClient,
    ToEthereumClientEnriched,
};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use std::str::FromStr;
use std::sync::Arc;

pub static PEER_BROADCAST_CONTRACT_ADDRESS: &'static str =
    "0xbf5a1966ed793a7ca90878701e410463836bb366";
pub static ORACLE_CONTRACT_ADDRESS: &'static str =
    "0x19800ab132174a00e2ab1434678bbc34554cb915";

// todo: this shouldn't be hardcoded -
// this should be taken from the peer-broadcast smart contract.
pub static DEMO_PEER_SET_SMART_CONTRACT_ADDRESS: &'static str =
    "0xc0d18d2a4129fec8095a1eb19ef14cc88200a4ac";

pub static CHAIN_ID: u64 = 31337u64;

pub static ORGANISATION_ONE_ADDR: &'static str =
    "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b";
pub static ORGANISATION_TWO_ADDR: &'static str =
    "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e";

pub fn demo_organisation_one(
) -> Result<Arc<ExecutingOrganisation>> {
    let wallet_address = "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969";
    let name = "peer-1";

    create_demo_organisation(name, wallet_address)
}

pub fn demo_organisation_two(
) -> Result<Arc<ExecutingOrganisation>> {
    let wallet_address = "d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e";
    let name = "peer-2";

    create_demo_organisation(name, wallet_address)
}

pub fn demo_peer_one_address() -> Address {
    Address::from_str(
        "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b",
    )
    .unwrap()
}

pub fn demo_peer_two_address() -> Address {
    Address::from_str(
        "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e",
    )
    .unwrap()
}

pub fn demo_peer_set_with_two_peers() -> Result<PeerSet> {
    Ok(PeerSet {
        peers: vec![
            Peer {
                ethereum_address: demo_peer_one_address(),
            },
            Peer {
                ethereum_address: demo_peer_two_address(),
            },
        ],
    })
}

// todo: this should be a real pointer.
pub fn demo_graph_ipfs_pointer() -> String {
    "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"
        .to_string()
}

pub fn demo_updated_ipfs_pointer() -> String {
    return "https://ipfs.io/ipfs/updated".to_string();
}

pub fn demo_peer_ipfs_pointer() -> String {
    "https://ipfs.io/ipfs/Q43q3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"
        .to_string()
}

pub fn create_demo_client(
    executing_organisation: Arc<ExecutingOrganisation>,
) -> Result<EnrichedEthereumClient> {
    let base_client = create_base_demo_client(
        executing_organisation.clone(),
    )?;
    let enriched_client = base_client.to_enriched_client(
        PEER_BROADCAST_CONTRACT_ADDRESS,
        ORACLE_CONTRACT_ADDRESS,
    )?;

    Ok(enriched_client)
}

pub fn create_base_demo_client(
    executing_organisation: Arc<ExecutingOrganisation>,
) -> Result<Arc<EthereumClient>> {
    let ethereum_client =
        ethereum_client::crate_local_ethereum_client(
            executing_organisation.clone(),
        )?;

    let ethereum_client = Arc::new(ethereum_client);
    Ok(ethereum_client)
}

pub fn shared_init() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::try_init()?;
    Ok(())
}

fn create_demo_organisation(
    name: &str,
    wallet_address: &str,
) -> Result<Arc<ExecutingOrganisation>> {
    let wallet = wallet_address
        .parse::<LocalWallet>()?
        .with_chain_id(CHAIN_ID);

    Ok(Arc::new(ExecutingOrganisation {
        organisation: Organisation {
            name: name.to_string(),
            ethereum_address: wallet.address(),
        },
        wallet,
    }))
}

use crate::ethereum_integration_test_constants::{WALLET_ONE, WALLET_THREE, WALLET_TWO};
use ethers_signers::Signer;
use organisation::core::ethereum::crate_local_ethereum_client;
use organisation::core::protocol::Peer;

mod ethereum_integration_test_constants {
    use ethers_signers::LocalWallet;
    use lazy_static::lazy_static;
    use organisation::core::ethereum::local_wallet;

    pub const WALLET_PK_ONE: &str =
        "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969";

    pub const WALLET_PK_TWO: &str =
        "d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e";

    pub const WALLET_PK_THREE: &str =
        "10c67ed269ed42da29eacc438e478ca74e3ebee11d200df324bc8cf89720fb65";

    lazy_static! {
        pub static ref WALLET_ONE: LocalWallet = local_wallet(WALLET_PK_ONE);
        pub static ref WALLET_TWO: LocalWallet = local_wallet(WALLET_PK_TWO);
        pub static ref WALLET_THREE: LocalWallet = local_wallet(WALLET_PK_THREE);
    }
}

#[tokio::test]
async fn peerset_smart_contract_integration_test() {
    let eth_client = crate_local_ethereum_client(WALLET_ONE.clone()).unwrap();
    let peer_one = &WALLET_ONE.address();
    let peer_two = &WALLET_TWO.address();
    let peer_three = &WALLET_THREE.address();

    let peers = vec![
        Peer::from_address(peer_one),
        Peer::from_address(peer_two),
        Peer::from_address(peer_three),
    ];
    let permission_graph_cid = "ipfs:://cid-1".to_string();

    let sc = eth_client
        .deploy_peer_set_smart_contract(peers, permission_graph_cid)
        .await
        .expect("PeerSet smart contract deployment by peer 1 should succeed");

    let eth_client_2 = crate_local_ethereum_client(WALLET_TWO.clone()).unwrap();
    let cid = "ipfs:://cid-2".to_string();
    eth_client_2
        .propose_change(sc.address(), cid.clone())
        .await
        .expect("Proposing a change by peer 2 should succeed");

    let eth_client_3 = crate_local_ethereum_client(WALLET_THREE.clone()).unwrap();
    eth_client_3
        .approve_change(sc.address(), cid.clone())
        .await
        .expect("Approving a change by peer 3 should succeed");

    let current_version = eth_client
        .current_version(sc.address())
        .await
        .expect("Fetching latest version should succeed");

    assert_eq!(current_version, cid)
}

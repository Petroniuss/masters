use crate::core::protocol::{BlockchainEvent, CommandEvent, Peer};
use crate::errors::Result;
use crate::ipfs::ipfs_client::CID;
use crate::poc::shared;
use crate::transport::ethereum::peer_set_smart_contract::{
    PeerSetPermissionGraphUpdatedFilter, PeerSetSmartContract, PeerSetSmartContractEvents,
};
use ethers::abi::{Token, Tokenizable};
use ethers::contract::stream::EventStream;
use ethers::contract::ContractError;
use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::prelude::{FilterWatcher, Log};
use ethers::providers::{Http, Provider, StreamExt, SubscriptionStream};
use ethers::types::Address;
use ethers_providers::{JsonRpcClient, PendingTransaction};
use ethers_signers::{LocalWallet, Signer};
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use log::{info, warn};
use std::ops::Add;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

/// EthereumFacade communicates asynchronously with Protocol through sender.
/// It notifies about Protocol about events from blockchain.
pub trait EthereumFacade: Send {
    fn async_create_peerset(&self, peers: Vec<Peer>, permission_graph_cid: CID);

    fn async_propose_change(&self, peerset_address: String, permission_graph_cid: CID);

    fn async_approve_change(&self, peerset_address: String, permission_graph_cid: CID);

    fn subscribe_to_peerset_events(&self, peerset_address: String);
}

pub struct EthereumFacadeImpl {
    pub sender: Sender<BlockchainEvent>,
    pub ethereum_client: Arc<EthereumClient>,
}

impl EthereumFacadeImpl {
    pub fn new(wallet: LocalWallet, sender: Sender<BlockchainEvent>) -> EthereumFacadeImpl {
        let eth_client = crate_local_ethereum_client(wallet).expect("should succeed");

        Self {
            sender,
            ethereum_client: Arc::new(eth_client),
        }
    }
}

pub trait AddressToString {
    fn to_full_string(&self) -> String;
}

impl AddressToString for Address {
    fn to_full_string(&self) -> String {
        return format!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::ethereum::{crate_local_ethereum_client, AddressToString};
    use crate::core::protocol::Peer;
    use crate::on_chain::ethereum_client::EthereumClient;
    use crate::poc::shared::CHAIN_ID;
    use ethers::types::Address;
    use ethers_signers::{LocalWallet, Signer};
    use std::str::FromStr;

    #[test]
    fn test_address_string_mapping() {
        let addr_string = "0xd13cd15657a8d1140690067a3a54fcf7a6ad50b9";
        let address = Address::from_str(addr_string).unwrap();

        assert_eq!(
            address.to_full_string(),
            "0xd13cd15657a8d1140690067a3a54fcf7a6ad50b9".to_string(),
        );
    }

    #[ignore]
    #[tokio::test]
    ///
    /// This test is ignored because it requires a local ethereum node to be running.
    /// It would be useful to have such a test as part of CI to make sure that this core part works.
    async fn propose_change() {
        fn local_wallet(wallet_address: &str) -> LocalWallet {
            wallet_address
                .parse::<LocalWallet>()
                .expect("should be valid wallet")
                .with_chain_id(CHAIN_ID)
        }

        let wallet_1 =
            local_wallet("2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969");

        let wallet_2 =
            local_wallet("d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e");

        // 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b
        let eth_client = crate_local_ethereum_client(wallet_1.clone()).expect("should succeed");

        let peer_one = &wallet_1.address();
        let peer_two = &wallet_2.address();
        let x = eth_client
            .deploy_peer_set_smart_contract(
                vec![
                    Peer {
                        blockchain_address: peer_one.to_full_string(),
                    },
                    Peer {
                        blockchain_address: peer_two.to_full_string(),
                    },
                ],
                "cid".to_string(),
            )
            .await
            .unwrap();

        // fails :/
        let address = x.address().to_full_string();
        eth_client
            .propose_change(address.parse().unwrap(), "cid-1".to_string())
            .await;
    }
}

impl EthereumFacade for EthereumFacadeImpl {
    fn async_create_peerset(&self, peers: Vec<Peer>, permission_graph_cid: CID) {
        let sender = self.sender.clone();
        let client = self.ethereum_client.clone();
        spawn(async move {
            let smart_contract = client
                .deploy_peer_set_smart_contract(peers.clone(), permission_graph_cid.clone())
                .await
                .expect("PeerSetSmartContract deployment should succeed");

            info!("SC address: {}", smart_contract.address().to_full_string());
            sender
                .send(BlockchainEvent::NewPeersetCreated {
                    peers,
                    permission_graph_cid,
                    peerset_address: smart_contract.address().to_full_string(),
                })
                .await
                .expect("should succeed");
        });
    }

    fn async_propose_change(&self, peerset_address: String, permission_graph_cid: CID) {
        let address = peerset_address.parse::<Address>().unwrap();
        let client = self.ethereum_client.clone();
        spawn(async move {
            client.propose_change(address, permission_graph_cid).await;
        });
    }

    fn async_approve_change(&self, peerset_address: String, permission_graph_cid: CID) {
        let address = peerset_address.parse::<Address>().unwrap();
        let client = self.ethereum_client.clone();
        spawn(async move {
            client.approve_change(address, permission_graph_cid).await;
        });
    }

    fn subscribe_to_peerset_events(&self, peerset_address: String) {
        let address = peerset_address.parse::<Address>().unwrap();
        let sender = self.sender.clone();
        let client = self.ethereum_client.clone();
        spawn(async move {
            info!("Subscribing to events for peerset: {}", peerset_address);
            client.subscribe_to_events(address, sender).await;
        });
    }
}

pub struct EthereumClient {
    pub ethereum_middleware: Arc<EthereumMiddleware>,
}

impl EthereumClient {
    async fn deploy_peer_set_smart_contract(
        &self,
        peers: Vec<Peer>,
        permission_graph_cid: CID,
    ) -> Result<PeerSetSmartContract<EthereumMiddleware>> {
        let peer_addresses: Vec<Address> = peers
            .into_iter()
            .map(|peer| peer.blockchain_address.parse::<Address>())
            .map(|e: _| e.expect("should be valid address"))
            .collect();

        let constructor_args = vec![
            peer_addresses.into_token(),
            permission_graph_cid.into_token(),
        ];

        let constructor_args = Token::Tuple(constructor_args);

        let contract_deployer =
            PeerSetSmartContract::deploy(self.ethereum_middleware.clone(), constructor_args)?;

        let peer_set_smart_contract = contract_deployer.send().await?;

        Ok(peer_set_smart_contract)
    }

    async fn propose_change(&self, peerset_address: Address, cid: CID) {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address, middleware);

        let call = sc.propose_permission_graph_change(cid.clone());
        println!(
            "Proposing a change with cid {} to peerset {}, {:?}",
            cid, peerset_address, call
        );
        let pending_tx_result = call.send().await;

        let pending_tx = decode_err(pending_tx_result);

        let _completed_tx = pending_tx.confirmations(1).await.unwrap().unwrap();
        info!(
            "Proposed a change with cid {} to peerset {}",
            cid, peerset_address
        );
    }

    async fn approve_change(&self, peerset_address: Address, cid: CID) {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let call = sc.submit_peer_vote(cid.clone(), true);
        let pending_tx = call.send().await;
        let pending_tx = decode_err(pending_tx);

        let _completed_tx = pending_tx.confirmations(1).await.unwrap().unwrap();
        info!(
            "Approved a change with cid {} to peerset {}",
            cid, peerset_address
        );
    }

    async fn subscribe_to_events(&self, peerset_address: Address, sender: Sender<BlockchainEvent>) {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let events = sc.events();
        let mut stream = events.stream().await.unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(v) => {
                    info!("PeerSetSmartContractEvent: {:?}", v);
                    match v {
                        PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRequestFilter(
                            e,
                        ) => {
                            sender
                                .send(BlockchainEvent::NewChangeProposed {
                                    peerset_blockchain_address: peerset_address.to_full_string(),
                                    proposed_by: Peer {
                                        blockchain_address: e
                                            .peer_requesting_change
                                            .to_full_string(),
                                    },
                                    new_permission_graph_cid: e
                                        .proposed_peer_set_permission_graph_ipfs_pointer,
                                })
                                .await
                                .unwrap();
                        }
                        PeerSetSmartContractEvents::PeerSetPermissionGraphUpdatedFilter(
                            PeerSetPermissionGraphUpdatedFilter {
                                peer_requesting_change,
                                updated_peer_set_permission_graph_ipfs_pointer,
                            },
                        ) => {
                            sender
                                .send(BlockchainEvent::ChangeAccepted {
                                    peerset_address: peerset_address.to_full_string(),
                                    new_permission_graph_cid:
                                        updated_peer_set_permission_graph_ipfs_pointer,
                                })
                                .await
                                .unwrap();
                        }

                        PeerSetSmartContractEvents::PeerSetPermissionGraphVoteReceivedFilter(_) => {
                        }
                        PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRejectedFilter(
                            _e,
                        ) => {}
                    }
                }
                Err(err) => {
                    warn!(
                        "Error during fetching events from peerset_smart_contract: {}, err: {}",
                        peerset_address, err
                    )
                }
            }
        }
    }
}

fn decode_err(
    pending_tx: std::result::Result<PendingTransaction<Http>, ContractError<EthereumMiddleware>>,
) -> PendingTransaction<Http> {
    match pending_tx {
        Ok(e) => e,
        Err(e) => match e {
            ContractError::Revert(_) => {
                let x = e.decode_revert::<String>();
                panic!("Transaction reverted: {:?}", x);
            }
            _ => {
                panic!("Infra problem: {:?}", e);
            }
        },
    }
}

type EthereumMiddleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub fn crate_local_ethereum_client(wallet: LocalWallet) -> Result<EthereumClient> {
    let provider = create_local_http_provider()?;
    let middleware = create_local_ethereum_middleware(provider, wallet.clone())?;
    let middleware = Arc::new(middleware);

    let client = EthereumClient {
        ethereum_middleware: middleware.clone(),
    };

    Ok(client)
}

fn create_local_http_provider() -> Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    Ok(provider)
}

pub fn create_local_ethereum_middleware(
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<EthereumMiddleware> {
    let provider = SignerMiddleware::new(provider.clone(), wallet.clone());

    let provider = NonceManagerMiddleware::new(provider.clone(), wallet.address());

    // let gas_oracle = EthGasStation::new(None);
    // let provider = GasOracleMiddleware::new(provider, gas_oracle);

    Ok(provider)
}

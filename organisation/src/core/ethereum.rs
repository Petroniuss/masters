use crate::core::ipfs::CID;
use crate::core::protocol::{BlockchainEvent, Peer};
use crate::errors::Result;
use crate::transport::ethereum::peer_set_smart_contract::{
    PeerSetPermissionGraphUpdatedFilter, PeerSetSmartContract, PeerSetSmartContractEvents,
};
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use ethers::abi::{Token, Tokenizable};
use ethers::contract::ContractError;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Provider, StreamExt};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use log::{info, warn};
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Sender;

pub static CHAIN_ID: u64 = 31337u64;

/// EthereumFacade communicates asynchronously with `core::Protocol` through `tokio::sync::mpsc::channel`.
pub trait EthereumFacade: Send {
    fn async_create_peerset(&self, peers: Vec<Peer>, permission_graph_cid: CID);

    fn async_propose_change(&self, peerset_address: String, permission_graph_cid: CID);

    fn async_propose_cross_peerset_change(
        &self,
        peerset_address: String,
        this_peerset_cid: CID,
        other_peerset_address: String,
        other_peerset_cid: CID,
    );

    fn async_approve_change(&self, peerset_address: String, permission_graph_cid: CID);

    fn subscribe_to_peerset_events(&self, peerset_address: String);
}

pub struct EthereumFacadeImpl {
    pub sender: Sender<BlockchainEvent>,
    pub ethereum_client: Arc<EthereumClient>,
}

impl EthereumFacadeImpl {
    pub fn new(wallet: LocalWallet, sender: Sender<BlockchainEvent>) -> EthereumFacadeImpl {
        let eth_client = crate_local_ethereum_client(wallet).unwrap();

        Self {
            sender,
            ethereum_client: Arc::new(eth_client),
        }
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
            client
                .propose_change(address, permission_graph_cid)
                .await
                .unwrap();
        });
    }

    fn async_propose_cross_peerset_change(
        &self,
        peerset_address: String,
        this_peerset_cid: CID,
        other_peerset_address: String,
        other_peerset_cid: CID,
    ) {
        let peerset_address = peerset_address.parse::<Address>().unwrap();
        let other_peerset_address = other_peerset_address.parse::<Address>().unwrap();
        let client = self.ethereum_client.clone();
        spawn(async move {
            client
                .propose_cross_peerset_change(
                    peerset_address,
                    this_peerset_cid,
                    other_peerset_address,
                    other_peerset_cid,
                )
                .await
                .unwrap();
        });
    }

    fn async_approve_change(&self, peerset_address: String, permission_graph_cid: CID) {
        let address = peerset_address.parse::<Address>().unwrap();
        let client = self.ethereum_client.clone();
        spawn(async move {
            client
                .approve_change(address, permission_graph_cid)
                .await
                .unwrap();
        });
    }

    fn subscribe_to_peerset_events(&self, peerset_address: String) {
        let address = peerset_address.parse::<Address>().unwrap();
        let sender = self.sender.clone();
        let client = self.ethereum_client.clone();
        spawn(async move {
            client.subscribe_to_peerset_events(address, sender).await;
        });
    }
}

type EthereumMiddleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

/// Interacts with ethereum through `transport` layer.
pub struct EthereumClient {
    pub ethereum_middleware: Arc<EthereumMiddleware>,
}

impl EthereumClient {
    pub async fn deploy_peer_set_smart_contract(
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

    pub async fn propose_change(&self, peerset_address: Address, cid: CID) -> Result<()> {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address, middleware);

        let call = sc.propose_permission_graph_change(cid.clone());
        let pending_tx = call.send().await.map_err(parse_contract_error)?;

        let _completed_tx = pending_tx.confirmations(1).await?;
        Ok(())
    }

    pub async fn propose_cross_peerset_change(
        &self,
        peerset_address: Address,
        this_peerset_cid: CID,
        other_peerset_address: Address,
        other_peerset_cid: CID,
    ) -> Result<()> {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address, middleware);

        let call = sc.propose_cross_peerset_change(
            this_peerset_cid,
            other_peerset_cid,
            other_peerset_address,
        );
        let pending_tx = call.send().await.map_err(parse_contract_error)?;

        let _completed_tx = pending_tx.confirmations(1).await?;
        Ok(())
    }

    pub async fn approve_change(&self, peerset_address: Address, cid: CID) -> Result<()> {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let call = sc.submit_peer_vote(cid.clone(), true);
        let pending_tx = call.send().await.map_err(parse_contract_error)?;

        let _completed_tx = pending_tx.confirmations(1).await?;
        info!("Approved transaction is committed! {}", cid);

        if let Some(_rec) = _completed_tx {}

        Ok(())
    }

    pub async fn current_version(&self, peerset_address: Address) -> Result<CID> {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let current_cid = sc
            .current_peer_set_permission_graph_ipfs_pointer()
            .call()
            .await?;

        Ok(current_cid)
    }

    // todo: NIT: code sending events should be part of the facade
    pub async fn subscribe_to_peerset_events(
        &self,
        peerset_address: Address,
        sender: Sender<BlockchainEvent>,
    ) {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let events = sc.events();
        let mut stream = events.stream().await.unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(v) => match v {
                    PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRequestFilter(e) => {
                        sender
                            .send(BlockchainEvent::NewChangeProposed {
                                peerset_blockchain_address: peerset_address.to_full_string(),
                                proposed_by: Peer {
                                    blockchain_address: e.peer_requesting_change.to_full_string(),
                                },
                                new_permission_graph_cid: e
                                    .proposed_peer_set_permission_graph_ipfs_pointer,
                            })
                            .await
                            .unwrap();
                    }
                    PeerSetSmartContractEvents::PeerSetPermissionGraphUpdatedFilter(
                        PeerSetPermissionGraphUpdatedFilter {
                            peer_requesting_change: _,
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

                    PeerSetSmartContractEvents::CrossPeersetGraphChangeRequestFilter(v) => {
                        sender
                            .send(BlockchainEvent::NewCrossPeersetChangeProposed {
                                peerset_address: peerset_address.to_full_string(),
                                this_peerset_cid: v.this_peerset_proposed_cid,
                                other_peerset_cid: v.other_peerset_proposed_cid,
                                other_peerset_address: v.other_peerset.to_full_string(),
                            })
                            .await
                            .unwrap();
                    }

                    PeerSetSmartContractEvents::PeerSetPermissionGraphVoteReceivedFilter(v) => {
                        info!("PeerSetSmartContractEvent: {:?}, {}", v, &peerset_address);
                    }
                    PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRejectedFilter(v) => {
                        info!("PeerSetSmartContractEvent: {:?}, {}", v, &peerset_address)
                    }
                },
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

pub trait AddressToString {
    fn to_full_string(&self) -> String;
}

impl AddressToString for Address {
    fn to_full_string(&self) -> String {
        return format!("{:?}", self);
    }
}

fn parse_contract_error(err: ContractError<EthereumMiddleware>) -> eyre::Report {
    return match err {
        ContractError::Revert(_) => {
            let decoded = err.decode_revert::<String>();
            eyre!(
                "ContractError:Revert reason: {}",
                decoded.unwrap_or("".to_string())
            )
        }
        _ => eyre!("Unexpected error {:?}", err),
    };
}

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

    Ok(provider)
}

pub fn local_wallet(wallet_address: &str) -> LocalWallet {
    wallet_address
        .parse::<LocalWallet>()
        .expect("should be valid wallet")
        .with_chain_id(CHAIN_ID)
}

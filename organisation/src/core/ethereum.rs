use crate::core::ipfs::CID;
use crate::core::protocol::{BlockchainEvent, Peer};
use crate::errors::Result;
use crate::transport::ethereum::peer_set_smart_contract::{
    PeerSetPermissionGraphUpdatedFilter, PeerSetSmartContract, PeerSetSmartContractEvents,
};
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use ethers::abi::{AbiDecode, Token, Tokenizable};
use ethers::contract::ContractError;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Provider, StreamExt};
use ethers::types::Res::Call as ResCall;
use ethers::types::{Address, CallResult, H256, U64};
use ethers_providers::Middleware;
use ethers_signers::{LocalWallet, Signer};
use log::{info, warn};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Sender;

// todo: verify that all transactions completed successfully and if not report it and decode error!
// todo: make sure that we're returning results instead of unwrapping due!

// this depends on chain
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
    pub node_id: String,
}

impl EthereumFacadeImpl {
    pub fn new(
        node_id: String,
        wallet: LocalWallet,
        sender: Sender<BlockchainEvent>,
    ) -> EthereumFacadeImpl {
        let eth_client = crate_local_ethereum_client(node_id.clone(), wallet).unwrap();

        Self {
            sender,
            ethereum_client: Arc::new(eth_client),
            node_id,
        }
    }
}

impl EthereumFacade for EthereumFacadeImpl {
    fn async_create_peerset(&self, peers: Vec<Peer>, permission_graph_cid: CID) {
        let sender = self.sender.clone();
        let client = self.ethereum_client.clone();
        spawn(async move {
            let result = client
                .deploy_peer_set_smart_contract(peers.clone(), permission_graph_cid.clone())
                .await;

            if let Err(e) = result {
                warn!("{}", e);
                return;
            }

            let smart_contract = result.unwrap();
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
            let res = client.propose_change(address, permission_graph_cid).await;

            if let Err(e) = res {
                warn!("{}", e);
            }
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
            let res = client
                .propose_cross_peerset_change(
                    peerset_address,
                    this_peerset_cid,
                    other_peerset_address,
                    other_peerset_cid,
                )
                .await;

            if let Err(e) = res {
                warn!("{}", e);
            }
        });
    }

    fn async_approve_change(&self, peerset_address: String, permission_graph_cid: CID) {
        let address = peerset_address.parse::<Address>().unwrap();
        let client = self.ethereum_client.clone();
        spawn(async move {
            let res = client.approve_change(address, permission_graph_cid).await;
            if let Err(e) = res {
                warn!("{}", e);
            }
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
    pub node_id: String,
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

        info!("{} Proposing change with cid: {}..", self.node_id, cid);
        let completed_tx = pending_tx.confirmations(1).await?;
        if let Some(receipt) = completed_tx {
            let status = receipt.status.unwrap();
            let success = status == U64::one();
            if success {
                info!(
                    "{} Proposed change with cid: {} - success: {}",
                    self.node_id, cid, success,
                );
                Ok(())
            } else {
                let error_msg = self.trace_error(receipt.transaction_hash).await?;
                Err(eyre!(
                    "{} Proposing change with cid: {} - failed with: `{}`",
                    self.node_id,
                    cid,
                    error_msg
                ))
            }
        } else {
            Err(eyre!(
                "{} Proposed change with cid: {} - no transaction receipt",
                self.node_id,
                cid
            ))
        }
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
            this_peerset_cid.clone(),
            other_peerset_cid.clone(),
            other_peerset_address,
        );
        let pending_tx = call.send().await.map_err(parse_contract_error)?;
        let completed_tx = pending_tx.confirmations(1).await?;
        if let Some(receipt) = completed_tx {
            let status = receipt.status.unwrap();
            let success = status == U64::one();
            if success {
                info!(
                    "{} Proposed cross-peerset change with cids: [{}, {}] - success",
                    self.node_id, this_peerset_cid, other_peerset_cid,
                );
                Ok(())
            } else {
                let error_msg = self.trace_error(receipt.transaction_hash).await?;
                Err(eyre!(
                    "{} Proposed cross-peerset change with cids: [{}, {}] - failed with `{}`",
                    self.node_id,
                    this_peerset_cid,
                    other_peerset_cid,
                    error_msg
                ))
            }
        } else {
            Err(eyre!(
                "{} Proposed cross-peerset change with cids: [{}, {}] - no receipt",
                self.node_id,
                this_peerset_cid,
                other_peerset_cid,
            ))
        }
    }

    pub async fn approve_change(&self, peerset_address: Address, cid: CID) -> Result<()> {
        let middleware = self.ethereum_middleware.clone();
        let sc = PeerSetSmartContract::new(peerset_address.clone(), middleware);

        let call = sc.submit_peer_vote(cid.clone(), true);
        // some transactions were running out of gas - need to set a limit.
        let call = call.gas(200000);
        let pending_tx = call.send().await.map_err(parse_contract_error)?;

        info!("{} Approving change with cid: {}", self.node_id, cid);

        let completed_tx = pending_tx.confirmations(1).await?;
        if let Some(receipt) = completed_tx {
            let status = receipt.status.unwrap();
            let success = status == U64::one();
            if success {
                info!("{} Approved change with cid: {}", self.node_id, cid);
            } else {
                let error_msg = self.trace_error(receipt.transaction_hash).await?;
                info!(
                    "{} Approving change with cid: {}, failed with `{}`",
                    self.node_id, cid, error_msg
                );
                // this error is expected when a transaction has already been approved.
                if !error_msg.contains("There are no pending changes") {
                    return Err(eyre!(
                        "{} approving change with cid {} returned an unknown error: {}",
                        self.node_id,
                        cid,
                        error_msg
                    ));
                }
            }
            Ok(())
        } else {
            Err(eyre!("no receipt!"))
        }
    }

    async fn trace_error(&self, tx_hash: H256) -> Result<String> {
        let mut traces = self
            .ethereum_middleware
            .trace_transaction(tx_hash)
            .await
            .unwrap();

        let trace = traces.remove(0);
        return if let ResCall(CallResult { output, .. }) = trace.result.unwrap() {
            let decoded_error = String::decode(&output[4..])?;
            Ok(decoded_error)
        } else {
            Err(eyre!("failed to decode failure reason!"))
        };
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

                    PeerSetSmartContractEvents::PeerSetPermissionGraphVoteReceivedFilter(_v) => {}
                    PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRejectedFilter(_v) => {}
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
            let decoded = err.decode_revert::<String>().unwrap_or("".to_string());
            eyre!("ContractError:Revert reason: {}", decoded)
        }
        _ => eyre!("Unexpected error {:?}", err),
    };
}

pub fn crate_local_ethereum_client(node_id: String, wallet: LocalWallet) -> Result<EthereumClient> {
    let provider = create_local_http_provider()?;
    let middleware = create_local_ethereum_middleware(provider, wallet.clone())?;
    let middleware = Arc::new(middleware);

    let client = EthereumClient {
        ethereum_middleware: middleware.clone(),
        node_id,
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

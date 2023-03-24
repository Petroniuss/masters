use crate::bindings::peer_set_smart_contract::PeerSetSmartContract;
use crate::core::protocol::{BlockchainEvent, CommandEvent, Peer};
use crate::errors::Result;
use crate::ipfs::ipfs_client::CID;
use ethers::abi::{Token, Tokenizable};
use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Sender;

/// EthereumFacade communicates asynchronously with Protocol through sender.
/// It notifies about Protocol about events from blockchain.
pub trait EthereumFacade: Send {
    /// todo: not sure about this context thing.
    fn async_create_peerset(
        &self,
        peers: Vec<Peer>,
        permission_graph_cid: CID,
        context: CommandEvent,
    );
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

impl EthereumFacade for EthereumFacadeImpl {
    fn async_create_peerset(
        &self,
        peers: Vec<Peer>,
        permission_graph_cid: CID,
        context: CommandEvent,
    ) {
        let sender = self.sender.clone();
        let client = self.ethereum_client.clone();
        spawn(async move {
            let smart_contract = client
                .deploy_peer_set_smart_contract(peers.clone(), permission_graph_cid.clone())
                .await
                .expect("PeerSetSmartContract deployment should succeed");

            sender
                .send(BlockchainEvent::NewPeersetCreated {
                    peers,
                    permission_graph_cid,
                    peerset_address: smart_contract.address().to_string(),
                    context: Some(context),
                })
                .await
                .expect("should succeed");
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

        // todo: this should be removed from actual smart contract!
        let oracle_address = peer_addresses[0].clone();

        let constructor_args = vec![
            peer_addresses.into_token(),
            oracle_address.into_token(),
            permission_graph_cid.into_token(),
        ];

        let constructor_args = Token::Tuple(constructor_args);

        let contract_deployer =
            PeerSetSmartContract::deploy(self.ethereum_middleware.clone(), constructor_args)?;

        let peer_set_smart_contract = contract_deployer.send().await?;

        Ok(peer_set_smart_contract)
    }
}

type EthereumMiddleware = GasOracleMiddleware<
    NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
    EthGasStation,
>;

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

    let gas_oracle = EthGasStation::new(None);
    let provider = GasOracleMiddleware::new(provider, gas_oracle);

    Ok(provider)
}

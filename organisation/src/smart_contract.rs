use crate::bindings::PermissionGraph;
use async_trait::async_trait;
use color_eyre::Result;
use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::NonceManagerMiddleware;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider, StreamExt};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use log::info;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;

type EthersClient = GasOracleMiddleware<
    NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
    EthGasStation,
>;
type SmartContractClient = PermissionGraph<EthersClient>;

#[async_trait]
trait SmartContractService {
    async fn fetch_current_graph_version(&self) -> Result<String>;

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()>;

    async fn listen_for_new_events(&self) -> Result<()>;
}

#[allow(dead_code)]
struct SmartContractServiceImpl {
    smart_contract_address: Address,
    organisation_wallet: LocalWallet,
    organisation_name: String,
    client: Arc<EthersClient>,
    smart_contract_client: SmartContractClient,
}

static ORGANISATION_NAME: &'static str = "ORG_A";
static WALLET_PRIVATE_KEY: &'static str =
    "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969";
static SMART_CONTRACT_ADDRESS: &'static str = "0xad70d5f3490a793b308182f6a0e59ba16298f1ef";

impl SmartContractServiceImpl {
    #[allow(dead_code)]
    async fn new_for_local_setup() -> Result<impl SmartContractService> {
        let provider = SmartContractServiceImpl::create_local_http_provider()?;

        let chain_id = provider.get_chainid().await?;

        let organisation_wallet = WALLET_PRIVATE_KEY
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let smart_contract_address = Address::from_str(SMART_CONTRACT_ADDRESS)?;

        let client =
            SmartContractServiceImpl::create_ethers_client(provider, organisation_wallet.clone())?;

        let smart_contract_client = PermissionGraph::new(smart_contract_address, client.clone());

        let organisation_name = ORGANISATION_NAME.to_string();

        Ok(Self {
            smart_contract_address,
            organisation_wallet,
            organisation_name,
            client,
            smart_contract_client,
        })
    }

    fn create_local_http_provider() -> Result<Provider<Http>> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;
        Ok(provider)
    }

    fn create_ethers_client(
        provider: Provider<Http>,
        wallet: LocalWallet,
    ) -> Result<Arc<EthersClient>> {
        let provider = SignerMiddleware::new(provider.clone(), wallet.clone());

        let provider = NonceManagerMiddleware::new(provider.clone(), wallet.address());

        let gas_oracle = EthGasStation::new(None);
        let provider = GasOracleMiddleware::new(provider, gas_oracle);

        let client = Arc::new(provider);

        Ok(client)
    }
}

#[async_trait]
impl SmartContractService for SmartContractServiceImpl {
    async fn fetch_current_graph_version(&self) -> Result<String> {
        let result = self
            .smart_contract_client
            .get_latest_permission_graph_ipfs_pointer()
            .call()
            .await?;
        info!("Fetched current graph version: {}", result);

        Ok(result)
    }

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()> {
        info!("Proposing new graph version: {:?}", graph_ipfs_pointer);
        let tx = self.smart_contract_client.propose_permission_graph_change(
            self.organisation_name.clone(),
            graph_ipfs_pointer.to_string(),
        );

        let pending_tx = tx.send().await?;

        info!("pending_tx: {:?}", pending_tx);
        let receipt = pending_tx
            .confirmations(1)
            .await?
            .expect("transaction should have been mined");

        info!("tx_receipt: {:?}", receipt);
        Ok(())
    }

    async fn listen_for_new_events(&self) -> Result<()> {
        let events = self.smart_contract_client.events().from_block(0);

        // note that to get future events we call .stream
        // and for historical events we call .query
        // there's a feature request to change this;
        // https://github.com/gakonst/ethers-rs/issues/988
        let mut stream = events.stream().await?;

        while let Some(Ok(evt)) = stream.next().await {
            info!("event: {:?}", evt);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_contract::SmartContractService;
    use crate::smart_contract::SmartContractServiceImpl;

    use color_eyre::Result;
    use log::info;
    use pretty_assertions::assert_eq;
    use test_log::test;

    #[test(tokio::test)]
    #[ignore]
    async fn smart_contract_service_integration_test() -> Result<()> {
        let permission_graph_smart_contract =
            SmartContractServiceImpl::new_for_local_setup().await?;

        let _prev_version = permission_graph_smart_contract
            .fetch_current_graph_version()
            .await?;

        let proposed_version = "ipfs://1";
        permission_graph_smart_contract
            .propose_new_graph_version(proposed_version)
            .await?;

        let actual_version = permission_graph_smart_contract
            .fetch_current_graph_version()
            .await?;

        assert_eq!(proposed_version, actual_version);
        Ok(())
    }

    #[test(tokio::test)]
    #[ignore]
    async fn smart_contract_service_events_integration_test() -> Result<()> {
        info!("Starting smart_contract_service_events_integration_test");
        let permission_graph_smart_contract =
            SmartContractServiceImpl::new_for_local_setup().await?;

        permission_graph_smart_contract
            .listen_for_new_events()
            .await
    }
}

use crate::bindings::PermissionGraph;
use async_trait::async_trait;
use color_eyre::Result;

use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::NonceManagerMiddleware;
use ethers::middleware::SignerMiddleware;

use ethers::providers::{Http, Middleware, Provider};
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

    // todo: listen for events coming from the smart contract.
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
    "db1969650ae7c48058e828fd7dfabe337d1047c86c047a12240c51c74be27ba9";
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

        let prev_version = permission_graph_smart_contract
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
}

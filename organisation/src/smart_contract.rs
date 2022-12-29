use crate::bindings::PermissionGraph;
use color_eyre::Result;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer, Wallet};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use async_trait::async_trait;
use ethers::contract::builders::ContractCall;
use ethers::core::k256;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::SignerMiddleware;
use ethers::middleware::NonceManagerMiddleware;
use log::{info, Log};

#[async_trait]
trait PermissionGraphSmartContract {
    async fn fetch_current_graph_version(&self) -> Result<String>;

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()>;

    // todo:
    // add functions to propose a new version of the graph
    // listen for events coming from the smart contract.
}

type Client = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;
type SmartContractClient = PermissionGraph<Client>;

struct PermissionGraphSmartContractImpl {
    smart_contract_address: Address,
    organisation_wallet: LocalWallet,
    organisation_name: String,
    client: Arc<Client>,
    smart_contract_client: SmartContractClient,
}

impl PermissionGraphSmartContractImpl {
    fn new_for_local_setup() -> Result<impl PermissionGraphSmartContract> {
        let organisation_wallet =
            "db1969650ae7c48058e828fd7dfabe337d1047c86c047a12240c51c74be27ba9"
                .parse::<LocalWallet>()?;

        let smart_contract_address =
            Address::from_str("0xad70d5f3490a793b308182f6a0e59ba16298f1ef")?;

        let client = PermissionGraphSmartContractImpl::create_client(
            organisation_wallet.clone(),
        )?;

        let smart_contract_client =
            PermissionGraph::new(smart_contract_address, client.clone());

        let organisation_name = "ORG_TEST_A".to_string();

        Ok(Self {
            smart_contract_address,
            organisation_wallet,
            organisation_name,
            client,
            smart_contract_client,
        })
    }

    fn create_client(wallet: LocalWallet) -> Result<Arc<Client>> {
        let provider =
            Provider::<Http>::try_from("http://localhost:8545")?;

        let provider = SignerMiddleware::new(
            provider.clone(), wallet.clone()
        );

        let provider =
            NonceManagerMiddleware::new(provider.clone(), wallet.address());

        let client =
            Arc::new(provider);

        Ok(client)
    }
}

#[async_trait]
impl PermissionGraphSmartContract for PermissionGraphSmartContractImpl {
    async fn fetch_current_graph_version(&self) -> Result<String> {
        let result = self.smart_contract_client
            .get_latest_permission_graph_ipfs_pointer()
            .call()
            .await?;

        Ok(result)
    }

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()> {
        let mut call: ContractCall<Client, ()> = self.smart_contract_client
            .propose_permission_graph_change(
                self.organisation_name.clone(),
                graph_ipfs_pointer.to_string()
            );

        call.tx.set_from(self.organisation_wallet.address().clone());

        let pending_tx = call
            .send().await?;
        info!("pending: {:?}", pending_tx);

        let receipt = pending_tx.confirmations(6).await?;
        info!("ala: {:?}", receipt);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;
    use log::info;
    use test_log::test;
    use color_eyre::Result;
    use ethers::providers::{Http, Middleware, Provider};
    use ethers::types::Address;
    use ethers_signers::{LocalWallet, Signer};
    use pretty_assertions::assert_eq;
    use crate::bindings::PermissionGraph;
    use crate::smart_contract::PermissionGraphSmartContractImpl;
    use crate::smart_contract::PermissionGraphSmartContract;

    #[test(tokio::test)]
    async fn dummy() -> Result<()>{
        let provider =
            Provider::<Http>::try_from("http://localhost:8545")?;

        let chain_id = provider.get_chainid().await?;

        let wallet =
            "09301bfd6a72ac78aec018637765fe0e8e4159372698a23a296deada8471c70f"
                .parse::<LocalWallet>()?
                .with_chain_id(chain_id.as_u64());

        let smart_contract_address =
            Address::from_str("0xad70d5f3490a793b308182f6a0e59ba16298f1ef")?;

        let client = PermissionGraphSmartContractImpl::create_client(
            wallet.clone(),
        )?;

        let smart_contract_client = PermissionGraph::new(smart_contract_address, client.clone()
        );

        let mut call = smart_contract_client.propose_permission_graph_change(
            "a".to_string(), "b".to_string()
        );
        /// works! omg!
        call.tx.set_gas(10000000);
        call.tx.set_gas_price(10000000000 as i64);

        // let signature = wallet.sign_transaction(&mut call.tx).await?;
        //
        // let pending_tx = client.send_raw_transaction(
        //     call.tx.rlp_signed(&signature)
        // ).await?;
        //

        call.send().await?;

        Ok(())
    }

    #[test(tokio::test)]
    async fn example() -> Result<()> {
        let permission_graph_smart_contract =
            PermissionGraphSmartContractImpl::new_for_local_setup()?;

        let prev_version = permission_graph_smart_contract.fetch_current_graph_version().await?;
        info!("prev version: {}", prev_version);

        // let proposed_version = "ipfs://1";
        // permission_graph_smart_contract.propose_new_graph_version(proposed_version).await?;

        info!("fo");
        //
        // let actual_version = permission_graph_smart_contract.fetch_current_graph_version().await?;
        // info!("current_graph_version: {}", actual_version);
        //
        // assert_eq!(proposed_version, actual_version);

        Ok(())
    }
}

use crate::bindings::PermissionGraph;
use color_eyre::Result;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Wallet};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use async_trait::async_trait;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use log::{info, Log};

#[async_trait]
trait PermissionGraphSmartContract {
    async fn fetch_current_graph_version(&self) -> Result<String>;

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()>;

    // todo:
    // add functions to propose a new version of the graph
    // listen for events coming from the smart contract.
}

struct PermissionGraphSmartContractImpl {
    // todo: this is going to be tricky, it's going to change with each deployment :/
    // figure out how to automate it.
    smart_contract_address: Address,
    organisation_address: Address,
    organisation_name: String,
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    permission_graph_smart_contract: PermissionGraph<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl PermissionGraphSmartContractImpl {
    fn new_for_local_setup() -> Result<impl PermissionGraphSmartContract> {
        let organisation_address = Address::from_str("0x18695328462441f0265dfe1004Fc462057F15218")?;

        let organisation_wallet =
            "db1969650ae7c48058e828fd7dfabe337d1047c86c047a12240c51c74be27ba9".parse::<LocalWallet>()?;
        let smart_contract_address =
            Address::from_str("0xad70d5f3490a793b308182f6a0e59ba16298f1ef")?;

        let provider = Provider::<Http>::try_from("http://localhost:8545")?;
        let client= Arc::new(SignerMiddleware::new(
            provider.clone(), organisation_wallet.clone()
        ));

        let permission_graph_smart_contract = PermissionGraph::new(smart_contract_address, client.clone());
        let organisation_name = "ORG_TEST_A".to_string();

        Ok(Self {
            smart_contract_address,
            organisation_address,
            organisation_name,
            client,
            permission_graph_smart_contract,
        })
    }
}

#[async_trait]
impl PermissionGraphSmartContract for PermissionGraphSmartContractImpl {
    async fn fetch_current_graph_version(&self) -> Result<String> {
        let result = self.permission_graph_smart_contract
            .get_latest_permission_graph_ipfs_pointer()
            .call()
            .await?;

        Ok(result)
    }

    async fn propose_new_graph_version(&self, graph_ipfs_pointer: &str) -> Result<()> {
        let from = self.organisation_address.clone();
        let call = self.permission_graph_smart_contract
            .propose_permission_graph_change(
                self.organisation_name.clone(),
                graph_ipfs_pointer.to_string());

        let pending_tx = call
            .send().await?;
        info!("pending: {:?}", pending_tx);

        let receipt = pending_tx.confirmations(6).await?;
        info!("receipt: {:?}", receipt);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use log::info;
    use test_log::test;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;
    use crate::smart_contract::PermissionGraphSmartContractImpl;
    use crate::smart_contract::PermissionGraphSmartContract;

    #[test(tokio::test)]
    async fn example() -> Result<()> {
        let permission_graph_smart_contract =
            PermissionGraphSmartContractImpl::new_for_local_setup()?;

        let prev_version = permission_graph_smart_contract.fetch_current_graph_version().await?;
        info!("prev version: {}", prev_version);

        let proposed_version = "ipfs://1";
        permission_graph_smart_contract.propose_new_graph_version(proposed_version).await?;

        let actual_version = permission_graph_smart_contract.fetch_current_graph_version().await?;
        info!("current_graph_version: {}", actual_version);

        assert_eq!(proposed_version, actual_version);

        Ok(())
    }
}

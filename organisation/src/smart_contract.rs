use crate::bindings::PermissionGraph;
use color_eyre::Result;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
trait PermissionGraphSmartContract {
    async fn fetch_current_graph_version(&self) -> Result<String>;

    // todo:
    // add functions to propose a new version of the graph
    // listen for events coming from the smart contract.
}

struct PermissionGraphSmartContractImpl {
    // todo: this is going to be tricky, it's going to change with each deployment :/
    // figure out how to automate it.
    smart_contract_address: Address,
    application_address: Address,
    client: Arc<Provider<Http>>,
    permission_graph_smart_contract: PermissionGraph<Provider<Http>>,
}

impl PermissionGraphSmartContractImpl {
    fn new_for_local_setup() -> Result<impl PermissionGraphSmartContract> {
        let smart_contract_address =
            Address::from_str("0xad70d5f3490a793b308182f6a0e59ba16298f1ef")?;
        let application_address = Address::from_str("0x18695328462441f0265dfe1004Fc462057F15218")?;
        let client = Arc::new(Provider::<Http>::try_from("http://localhost:8545")?);
        let permission_graph_smart_contract = PermissionGraph::new(smart_contract_address, client.clone());

        Ok(Self {
            smart_contract_address,
            application_address,
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
}

#[cfg(test)]
mod tests {
    use crate::smart_contract::PermissionGraphSmartContractImpl;
    use crate::smart_contract::PermissionGraphSmartContract;
    use log::info;
    use test_log::test;
    use color_eyre::Result;

    #[test(tokio::test)]
    async fn example() -> Result<()> {
        let permission_graph_smart_contract =
            PermissionGraphSmartContractImpl::new_for_local_setup()?;

        let current_graph_version = permission_graph_smart_contract.fetch_current_graph_version().await?;
        info!("current_graph_version: {}", current_graph_version);

        Ok(())
    }
}

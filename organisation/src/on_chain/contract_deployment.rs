use crate::bindings;
use crate::data_model::organisation::ExecutingOrganisation;
use crate::data_model::peer_set::PeerSet;
use crate::errors::Result;
use crate::on_chain::ethereum_client::EthereumClient;
use bindings::peer_set_smart_contract::peer_set_smart_contract;

use ethers::abi::{Token, Tokenizable};

use ethers::types::Address;

use log::info;
use peer_set_smart_contract::PeerSetSmartContract;
use std::sync::Arc;

pub struct PeerSetContractDeploymentService {
    pub executing_organisation: Arc<ExecutingOrganisation>,
    pub ethereum_client: Arc<EthereumClient>,
}

impl PeerSetContractDeploymentService {
    pub async fn deploy_peer_set_contract(
        &self,
        peer_set: &PeerSet,
        oracle_address: Address, // todo: switch this to an actual contract to get more type safety.
        initial_graph_ipfs_pointer: String, // todo: as above.
    ) -> Result<()> {
        info!("Beginning deployment of the peer set smart contract...");
        let peer_addresses =
            peer_set.get_peer_ethereum_addresses();

        let constructor_args = vec![
            peer_addresses.into_token(),
            oracle_address.into_token(),
            initial_graph_ipfs_pointer.into_token(),
        ];

        let constructor_args = Token::Tuple(constructor_args);

        info!("Deploying the peer set smart contract with the following arguments: {:?}", constructor_args);

        // we need to somehow pass 3 separate parameters!
        let contract_deployer = PeerSetSmartContract::deploy(
            self.ethereum_client.clone(),
            constructor_args,
        )?;

        let peer_set_smart_contract =
            contract_deployer.send().await?;
        info!(
            "Deployed peer set smart contract: {:?}",
            peer_set_smart_contract
        );

        Ok(())
    }
}

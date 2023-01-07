use crate::bindings;
use crate::bindings::peer_broadcast::PeerBroadcast;
use crate::bindings::permission_verifier_oracle::PermissionVerifierOracle;
use crate::data_model::organisation::ExecutingOrganisation;
use crate::data_model::peer_set::PeerSet;
use crate::errors::Result;
use crate::on_chain::ethereum_client::EthereumClient;
use async_trait::async_trait;
use bindings::peer_set_smart_contract::peer_set_smart_contract;
use ethers::abi::{Token, Tokenizable};
use log::info;
use peer_set_smart_contract::PeerSetSmartContract;
use std::sync::Arc;

#[async_trait]
pub trait PeerSetSmartContractDeployment {
    async fn deploy_peer_set_smart_contract(
        &self,
        peer_set: &PeerSet,
        permission_verifier_oracle: &PermissionVerifierOracle<
            EthereumClient,
        >,
        initial_graph_ipfs_pointer: String, // todo: as above.
    ) -> Result<PeerSetSmartContract<EthereumClient>>;
}

#[async_trait]
pub trait OracleSmartContractDeployment {
    async fn deploy_permission_verifier_oracle(
        &self,
    ) -> Result<PermissionVerifierOracle<EthereumClient>>;
}

#[async_trait]
pub trait PeerBroadcastSmartContractDeployment {
    async fn deploy_peer_broadcast_smart_contract(
        &self,
    ) -> Result<PeerBroadcast<EthereumClient>>;
}

pub struct SmartContractDeploymentService {
    pub executing_organisation: Arc<ExecutingOrganisation>,
    pub ethereum_client: Arc<EthereumClient>,
}

#[async_trait]
impl PeerSetSmartContractDeployment
    for SmartContractDeploymentService
{
    async fn deploy_peer_set_smart_contract(
        &self,
        peer_set: &PeerSet,
        permission_verifier_oracle: &PermissionVerifierOracle<
            EthereumClient,
        >,
        initial_graph_ipfs_pointer: String,
    ) -> Result<PeerSetSmartContract<EthereumClient>> {
        info!("Beginning deployment of the peer set smart contract...");
        let peer_addresses =
            peer_set.get_peer_ethereum_addresses();

        let oracle_address =
            permission_verifier_oracle.address();

        let constructor_args = vec![
            peer_addresses.into_token(),
            oracle_address.into_token(),
            initial_graph_ipfs_pointer.into_token(),
        ];

        let constructor_args = Token::Tuple(constructor_args);

        info!("Deploying the peer set smart contract with the following arguments: {:?}", constructor_args);

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

        Ok(peer_set_smart_contract)
    }
}

#[async_trait]
impl OracleSmartContractDeployment
    for SmartContractDeploymentService
{
    async fn deploy_permission_verifier_oracle(
        &self,
    ) -> Result<PermissionVerifierOracle<EthereumClient>> {
        info!("Beginning deployment of the oracle smart contract...");
        let constructor_args = Token::Tuple(vec![]);
        let contract_deployer =
            PermissionVerifierOracle::deploy(
                self.ethereum_client.clone(),
                constructor_args,
            )?;

        let oracle_smart_contract =
            contract_deployer.send().await?;

        info!(
            "Deployed oracle smart contract: {:?}",
            oracle_smart_contract
        );

        Ok(oracle_smart_contract)
    }
}
#[async_trait]
impl PeerBroadcastSmartContractDeployment
    for SmartContractDeploymentService
{
    async fn deploy_peer_broadcast_smart_contract(
        &self,
    ) -> Result<PeerBroadcast<EthereumClient>> {
        info!("Beginning deployment of the peer broadcast contract...");
        let constructor_args = Token::Tuple(vec![]);
        let contract_deployer = PeerBroadcast::deploy(
            self.ethereum_client.clone(),
            constructor_args,
        )?;

        let peer_broadcast_smart_contract =
            contract_deployer.send().await?;

        info!(
            "Deployed peer broadcast smart contract: {:?}",
            peer_broadcast_smart_contract
        );

        Ok(peer_broadcast_smart_contract)
    }
}

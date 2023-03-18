use crate::bindings::peer_set_smart_contract::PeerSetSmartContract;
use crate::data_model::peer_set::PeerSet;
use crate::errors::Result;
use crate::on_chain::contract_deployment::PeerSetSmartContractDeployment;
use crate::on_chain::ethereum_client::{
    EnrichedEthereumClient, EthereumMiddleware,
};
use async_trait::async_trait;
use log::info;

#[async_trait]
pub trait PeerBroadcastService {
    async fn register_itself(
        &self,
        peer_ipfs_pointer: String,
    ) -> Result<()>;

    async fn register_peerset(
        &self,
        peer_set: &PeerSet,
        initial_graph_ipfs_pointer: String,
    ) -> Result<PeerSetSmartContract<EthereumMiddleware>>;
}

#[async_trait]
impl PeerBroadcastService for EnrichedEthereumClient {
    async fn register_itself(
        &self,
        peer_ipfs_pointer: String,
    ) -> Result<()> {
        info!("Registering itself with the peer broadcast smart contract...");
        let peer_broadcast = &self.peer_broadcast_sc;

        let call =
            peer_broadcast.register_peer(peer_ipfs_pointer);

        let pending_tx = call.send().await?;

        info!("pending_tx: {:?}", pending_tx);

        let _receipt =
            pending_tx.confirmations(1).await?.unwrap();

        info!("Registered itself.");
        Ok(())
    }

    async fn register_peerset(
        &self,
        peer_set: &PeerSet,
        initial_graph_ipfs_pointer: String,
    ) -> Result<PeerSetSmartContract<EthereumMiddleware>> {
        let peer_set_smart_contract = self
            .deploy_peer_set_smart_contract(
                peer_set,
                initial_graph_ipfs_pointer,
            )
            .await?;

        // let peer_broadcast = &self.peer_broadcast_sc;
        //
        // let call = peer_broadcast.register_peer_set(
        //     peer_set_smart_contract.address(),
        // );
        //
        // let pending_tx = call.send().await?;
        //
        // let _receipt =
        //     pending_tx.confirmations(1).await?.unwrap();

        Ok(peer_set_smart_contract)
    }
}

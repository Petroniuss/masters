use crate::bindings::peer_set_smart_contract::PeerSetSmartContract;

use crate::errors::Result;
use crate::on_chain::ethereum_client::{
    EnrichedEthereumClient, EthereumMiddleware,
};

use ethers::types::Address;
use log::info;
use std::str::FromStr;

pub struct PeerSetSmartContractService {
    pub smart_contract: PeerSetSmartContract<EthereumMiddleware>,
}

impl PeerSetSmartContractService {
    pub async fn propose_change(
        &self,
        changed_graph_ipfs: String,
    ) -> Result<()> {
        let call = self
            .smart_contract
            .propose_permission_graph_change(
                changed_graph_ipfs.clone(),
            );
        info!(
            "Proposing change to permission graph: {:?}, {}",
            call.tx, changed_graph_ipfs
        );

        let pending_tx = call.send().await?;
        info!("Proposed a change: {:?}", pending_tx);

        let _receipt =
            pending_tx.confirmations(1).await?.unwrap();
        info!("Receipt: {:?}", _receipt);

        Ok(())
    }

    pub fn address(&self) -> Address {
        self.smart_contract.address()
    }

    pub async fn log_past_events(&self) -> Result<()> {
        info!(
            "Retrieving events for peer-set smart contract {}",
            self.smart_contract.address()
        );

        let events = self
            .smart_contract
            .events()
            .from_block(0)
            .query()
            .await?;

        for event in events {
            info!("event: {:?}", event);
        }

        Ok(())
    }
}

pub trait PeerSetSmartContractServiceFromAddress {
    fn connect_to_peer_set_sc(
        &self,
        address: &str,
    ) -> Result<PeerSetSmartContractService>;
}

impl PeerSetSmartContractServiceFromAddress
    for EnrichedEthereumClient
{
    fn connect_to_peer_set_sc(
        &self,
        address: &str,
    ) -> Result<PeerSetSmartContractService> {
        let smart_contract = PeerSetSmartContract::new(
            Address::from_str(address)?,
            self.ethereum_middleware.clone(),
        );

        Ok(PeerSetSmartContractService { smart_contract })
    }
}

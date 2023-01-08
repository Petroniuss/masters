use crate::bindings::permission_verifier_oracle::PermissionGraphValidationRequestedFilter;
use crate::bindings::permission_verifier_oracle::{
    PermissionVerifierOracle, PermissionVerifierOracleEvents,
};

use crate::errors::Result;
use crate::on_chain::ethereum_client::{
    EnrichedEthereumClient, EthereumMiddleware,
};

use ethers::types::Address;

use log::info;

pub struct OracleService {
    smart_contract:
        PermissionVerifierOracle<EthereumMiddleware>,
}

impl OracleService {
    pub async fn validate_change(
        &self,
        request_id: [u8; 32],
        is_valid: bool,
    ) -> Result<()> {
        info!("Foo!");
        let call = self
            .smart_contract
            .submit_peer_validation(request_id, is_valid);
        info!("Submitting validation to oracle: {:?}", call.tx);

        let pending_tx = call.send().await?;
        info!("Sent validated change: {:?}", pending_tx);

        let _receipt =
            pending_tx.confirmations(1).await?.unwrap();
        info!("Receipt for validated change: {:?}", _receipt);

        Ok(())
    }

    pub async fn find_latest_oracle_validation_request(
        &self,
        _peer_set_sc_address: Address,
    ) -> Result<Option<PermissionGraphValidationRequestedFilter>>
    {
        info!(
            "Retrieving oracle events for peer-set smart contract {}",
            self.smart_contract.address()
        );

        let event = self
            .smart_contract
            .events()
            .from_block(0)
            .query()
            .await?
            .into_iter()
            .filter(|event| {
                match event {
                    PermissionVerifierOracleEvents::PermissionGraphChangeValidatedFilter(evt) => {
                        info!("event: {:?}", evt);
                        false
                    }
                    PermissionVerifierOracleEvents::PermissionGraphValidationRequestedFilter(evt) => {
                        info!("event: {:?}", evt);
                        true
                    }
                }
            })
            .next();

        Ok(match event {
            Some(event) => {
                match event {
                    PermissionVerifierOracleEvents::PermissionGraphValidationRequestedFilter(evt) => {
                        Some(evt)
                    }
                    _ => None
                }
            }
            None => None
        })
    }

    pub fn address(&self) -> Address {
        self.smart_contract.address()
    }
}

pub trait OracleServiceFromAddress {
    fn connect_to_oracle(&self) -> Result<OracleService>;
}

impl OracleServiceFromAddress for EnrichedEthereumClient {
    fn connect_to_oracle(&self) -> Result<OracleService> {
        let smart_contract =
            self.permission_verifier_oracle.clone();

        Ok(OracleService { smart_contract })
    }
}

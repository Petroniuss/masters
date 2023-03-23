use crate::bindings::peer_broadcast::PeerBroadcast;
use crate::bindings::permission_verifier_oracle::PermissionVerifierOracle;
use crate::data_model::organisation::ExecutingOrganisation;
use crate::errors::Result;
use ethers::middleware::gas_oracle::{EthGasStation, GasOracleMiddleware};
use ethers::middleware::NonceManagerMiddleware;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};

use std::str::FromStr;
use std::sync::Arc;

pub struct EthereumClient {
    pub executing_organisation: Arc<ExecutingOrganisation>,
    pub ethereum_middleware: Arc<EthereumMiddleware>,
}

pub struct EnrichedEthereumClient {
    pub executing_organisation: Arc<ExecutingOrganisation>,
    pub ethereum_middleware: Arc<EthereumMiddleware>,
    pub peer_broadcast_sc: PeerBroadcast<EthereumMiddleware>,
    pub permission_verifier_oracle: PermissionVerifierOracle<EthereumMiddleware>,
}

pub trait ToEthereumClientEnriched {
    fn to_enriched_client(
        &self,
        peer_broadcast_sc_address: &str,
        permission_verifier_oracle_address: &str,
    ) -> Result<EnrichedEthereumClient>;
}

impl ToEthereumClientEnriched for Arc<EthereumClient> {
    fn to_enriched_client(
        &self,
        peer_broadcast_sc_address: &str,
        permission_verifier_oracle_address: &str,
    ) -> Result<EnrichedEthereumClient> {
        let ethereum_middleware = self.ethereum_middleware.clone();

        let executing_organisation = self.executing_organisation.clone();

        let peer_broadcast_sc = PeerBroadcast::new(
            Address::from_str(peer_broadcast_sc_address)?,
            ethereum_middleware.clone(),
        );

        let permission_verifier_oracle = PermissionVerifierOracle::new(
            Address::from_str(permission_verifier_oracle_address)?,
            ethereum_middleware.clone(),
        );

        let enriched = EnrichedEthereumClient {
            executing_organisation,
            ethereum_middleware,
            peer_broadcast_sc,
            permission_verifier_oracle,
        };

        Ok(enriched)
    }
}

pub type EthereumMiddleware = GasOracleMiddleware<
    NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
    EthGasStation,
>;

// todo: this should use layered-configuration for everything but for now let's skip it.
pub fn crate_local_ethereum_client(
    executing_organisation: Arc<ExecutingOrganisation>,
) -> Result<EthereumClient> {
    let provider = create_local_http_provider()?;
    let wallet = executing_organisation.wallet.clone();
    let middleware = create_local_ethereum_middleware(provider, wallet)?;
    let middleware = Arc::new(middleware);

    let client = EthereumClient {
        executing_organisation,
        ethereum_middleware: middleware.clone(),
    };

    Ok(client)
}

fn create_local_http_provider() -> Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    Ok(provider)
}

pub fn create_local_ethereum_middleware(
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<EthereumMiddleware> {
    let provider = SignerMiddleware::new(provider.clone(), wallet.clone());

    let provider = NonceManagerMiddleware::new(provider.clone(), wallet.address());

    let gas_oracle = EthGasStation::new(None);
    let provider = GasOracleMiddleware::new(provider, gas_oracle);

    Ok(provider)
}

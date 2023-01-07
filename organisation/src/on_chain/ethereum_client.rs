use crate::data_model::organisation::ExecutingOrganisation;
use crate::errors::Result;
use ethers::middleware::gas_oracle::{
    EthGasStation, GasOracleMiddleware,
};
use ethers::middleware::NonceManagerMiddleware;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_signers::{LocalWallet, Signer};
use std::sync::Arc;

pub type EthereumClient = GasOracleMiddleware<
    NonceManagerMiddleware<
        SignerMiddleware<Provider<Http>, LocalWallet>,
    >,
    EthGasStation,
>;

// todo: this should use layered-configuration for everything but for now let's skip it.

pub fn crate_local_ethereum_client(
    executing_organisation: Arc<ExecutingOrganisation>,
) -> Result<Arc<EthereumClient>> {
    let provider = create_local_http_provider()?;
    let wallet = executing_organisation.wallet.clone();
    let client =
        create_local_ethereum_client(provider, wallet)?;
    let client = Arc::new(client);
    Ok(client)
}

fn create_local_http_provider() -> Result<Provider<Http>> {
    let provider =
        Provider::<Http>::try_from("http://localhost:8545")?;
    Ok(provider)
}

pub fn create_local_ethereum_client(
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<EthereumClient> {
    let provider =
        SignerMiddleware::new(provider.clone(), wallet.clone());

    let provider = NonceManagerMiddleware::new(
        provider.clone(),
        wallet.address(),
    );

    let gas_oracle = EthGasStation::new(None);
    let provider =
        GasOracleMiddleware::new(provider, gas_oracle);

    Ok(provider)
}

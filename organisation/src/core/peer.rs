use crate::core::ethereum::local_wallet;
use crate::core::grpc::OrganisationDevService;
use crate::core::protocol::ProtocolFacade;
use crate::errors::Result;

use crate::transport::grpc::command::organisation_dev_server::OrganisationDevServer;

use ethers::addressbook::Address;
use ethers_signers::Signer;
use log::info;

use tonic::transport::Server;

pub async fn run_with_configuration(configuration: Configuration) -> Result<()> {
    let addr = format!("[::1]:{}", configuration.port).parse()?;
    info!("Running on: {}", addr);

    let wallet = local_wallet(&configuration.wallet_pk);
    let protocol_facade = ProtocolFacade::new(wallet);
    let organisation_service = OrganisationDevService::new(protocol_facade);
    let organisation_server = OrganisationDevServer::new(organisation_service);

    Server::builder()
        .add_service(organisation_server)
        .serve(addr)
        .await?;

    Ok(())
}

pub fn load_configuration_from_env() -> Configuration {
    let profile = std::env::var("ORG_PROFILE").expect("ORG_PROFILE should be set");

    match profile.as_str() {
        "peer_1" => peer_1_configuration(),
        "peer_2" => peer_2_configuration(),
        "peer_3" => peer_3_configuration(),
        "peer_4" => peer_4_configuration(),
        "peer_5" => peer_5_configuration(),
        _ => {
            panic!("Unknown profile {}", profile);
        }
    }
}

#[derive(Clone)]
pub struct Configuration {
    pub port: String,
    pub wallet_pk: String,
}

impl Configuration {
    pub fn address(&self) -> Address {
        return local_wallet(self.wallet_pk.as_str()).address();
    }

    pub fn local_connection_str(&self) -> String {
        format!("http://[::1]:{}", self.port)
    }
}

pub fn peer_1_configuration() -> Configuration {
    return Configuration {
        port: "50051".to_string(),
        wallet_pk: "2834824554106f1a77dd199dfc5456cb40091f560b3b3d2d3417bb04d04bd969".to_string(),
    };
}

pub fn peer_2_configuration() -> Configuration {
    return Configuration {
        port: "50052".to_string(),
        wallet_pk: "d2ef8f291387de16e7ae1875f80d3d31a4b7e6687294862ff9793d584f933a5e".to_string(),
    };
}

pub fn peer_3_configuration() -> Configuration {
    return Configuration {
        port: "50053".to_string(),
        wallet_pk: "10c67ed269ed42da29eacc438e478ca74e3ebee11d200df324bc8cf89720fb65".to_string(),
    };
}

pub fn peer_4_configuration() -> Configuration {
    return Configuration {
        port: "50054".to_string(),
        wallet_pk: "526a63ba7b1c3ad4a3f5cc923b30f2ac9eb5b039a00d6b4990459a38d7f56743".to_string(),
    };
}

pub fn peer_5_configuration() -> Configuration {
    return Configuration {
        port: "50055".to_string(),
        wallet_pk: "5096f8ad0d4fd8906fd9e574fc5bc9d1623d1c0ca257f3aca8a3bee68f9fda8b".to_string(),
    };
}

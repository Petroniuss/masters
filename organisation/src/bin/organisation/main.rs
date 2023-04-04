use organisation::core::peer;
use organisation::core::peer::load_configuration_from_env;
use organisation::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = load_configuration_from_env();
    peer::run_with_configuration(configuration).await
}

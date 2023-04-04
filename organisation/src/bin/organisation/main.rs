use organisation::core::peer;
use organisation::core::peer::load_configuration_from_env;
use organisation::errors::Result;
use organisation::shared::shared::init;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;
    let configuration = load_configuration_from_env();
    peer::run_with_configuration(configuration).await
}

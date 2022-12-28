use std::fs::canonicalize;
use ethers::prelude::Abigen;
use color_eyre::eyre::Result;
use log::info;

const CONTRACT_ABI_JSON_SOURCE: &str = "../blockchain/build/contracts/PermissionGraph.json";
const BINDINGS_OUTPUT_PATH: &str = "./src/bindings.rs";

/// we could also consider having a task that compiles the contract,
/// for simplicity let's assume we have a compiled contract.
/// assumes that task is executed from organisation directory.
fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LOG", "info");
    color_eyre::install()?;
    sensible_env_logger::init!();

    let current_dir = std::env::current_dir()?;

    let directory_string = current_dir
        .to_str()
        .unwrap()
        .to_string();

    let directory_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    info!("Generating rust bindings for {} to {}",
        canonicalize(CONTRACT_ABI_JSON_SOURCE)?.to_str().unwrap(),
        canonicalize(BINDINGS_OUTPUT_PATH)?.to_str().unwrap()
    );

    if directory_name != "organisation" {
        panic!("gen-bindings task must be executed from organisation directory! \
        Was executed from {}", directory_string);
    }

    Abigen::from_file(CONTRACT_ABI_JSON_SOURCE)
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(BINDINGS_OUTPUT_PATH)
}


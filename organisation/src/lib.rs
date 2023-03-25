pub mod core;
pub mod transport;

/// these modules should be removed sooner or later :)
pub mod data_model;
pub mod errors;
pub mod ipfs;
pub mod on_chain;
pub mod poc;

pub fn hello() {
    println!("Hello, world!");
}

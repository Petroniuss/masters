pub mod core;
pub mod transport;

/// these modules should be removed sooner or later :)
pub mod errors;
pub mod ipfs;
pub mod shared;

pub fn hello() {
    println!("Hello, world!");
}

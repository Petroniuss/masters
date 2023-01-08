mod bindings;
mod permission_graph;
mod smart_contract;

/// Goal for today:
/// Let's now write the off-chain code that interacts with the smart contract
/// Things that we need:
/// - deploy multiple smart contracts but initialized with different arguments
/// - interact with the smart contract:
///   - propose a change
///   - validate the change
/// - update docker-compose to run hello-world setup:
///   - ideally this would be an **example** or maybe a separate **bin** target?
///
/// Using examples:
///   - create lib crate.
///   - create examples folder.
///   - create bin crate consuming lib crate.
///   - Goal: https://doc.rust-lang.org/cargo/guide/project-layout.html
///
/// Stretch:
/// - off-chain code assembling knowledge from multiple smart contracts,
/// - that's able to respond to queries whether given operation is allowed.
///
/// - use dockerized-rust image to build and run the code in CI pipeline:
///     - switch from using rust-cache to pushing each layer to docker registry
///     - switch to Docker BuildKit in CI pipeline and integrate with docker-registry.
///
fn main() {
    println!("FooBaz!");
}

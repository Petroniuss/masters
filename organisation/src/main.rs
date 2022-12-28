mod permission_graph;

fn main() {
    println!("Master's");
}
// next step - interact with the smart contract
// we need to generate rust bindings.
// let's create a cargo task that does it.



// next step - interact with IPFS

// Need a way to:
// - bootstrap IPFS
// - boostrap the permission graph smart contract
// - initialize the permission graph
// - visualize the graph live.
// would be nice if I could do everything with docker compose





// hmm but I need to be able to interact with some pieces - how?
// the easiest would be to expose some simple API and use curl/postman/scripts?
// or maybe some dummy front-end - hopefully compile rust to wasm and serve that?
// trunk/yew seems to be the way to go.


// Organisation should be both an HTTP server so that other organisations can communicate with it
// and be an interactive CLI so that we can play with it easily.
// interactive CLI should be behind -i flag.

// let's start with creating a PoC representation of permissions graph.
// use-cases:
//    - organisation adds a user.
//    - organisation adds a group.
//    - organisation adds a space.
//    - organisation adds a permission.
//    - organisation publishes a new version of the graph on blockchain
//      to a hardcoded smart contract.
//    - verify if the graph is valid.

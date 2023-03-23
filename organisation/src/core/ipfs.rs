use crate::core::protocol::IPFSEvent;
use crate::errors::Result;
use crate::grpc::command::{
    Edge, Edges, Node, NodeType, PermissionGraph,
};
use crate::ipfs::ipfs_client::CID;
use std::collections::HashMap;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn load_permission_graph(
        &self,
        cid: CID,
        peerset_address: String,
    );
}

use crate::core::protocol::CommandEvent;
use crate::grpc::command::PermissionGraph;
use crate::ipfs::ipfs_client::CID;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String);

    /// Context is passed during asynchronous callback
    fn async_save_permission_graph(&self, permission_graph: PermissionGraph, context: CommandEvent);
}

use crate::core::protocol::CommandEvent;
use crate::grpc::command::PermissionGraph;
use crate::ipfs::ipfs_client::CID;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String);

    /// Context is passed during asynchronous callback
    fn async_save_permission_graph(&self, permission_graph: PermissionGraph, context: CommandEvent);
}

pub struct NoOpIPFSFacade {}

impl NoOpIPFSFacade {
    pub fn new() -> Self {
        return Self {};
    }
}

impl IPFSFacade for NoOpIPFSFacade {
    fn async_load_permission_graph(&self, _cid: CID, _peerset_address: String) {
        todo!()
    }

    fn async_save_permission_graph(
        &self,
        _permission_graph: PermissionGraph,
        _context: CommandEvent,
    ) {
        todo!()
    }
}

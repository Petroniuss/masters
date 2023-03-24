use crate::core::protocol::{CommandEvent, IPFSEvent};
use crate::grpc::command::PermissionGraph;
use crate::ipfs::ipfs_client::CID;
use crate::poc::shared;

use tokio::sync::mpsc::Sender;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String);

    /// Context is passed during asynchronous callback
    fn async_save_permission_graph(&self, permission_graph: PermissionGraph, context: CommandEvent);
}

pub struct CheatingIPFSFacade {
    pub ipfs_sender: Sender<IPFSEvent>,
}

impl CheatingIPFSFacade {
    pub fn new(ipfs_sender: Sender<IPFSEvent>) -> Self {
        return Self { ipfs_sender };
    }
}

impl IPFSFacade for CheatingIPFSFacade {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String) {
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            sender
                .send(IPFSEvent::PermissionGraphLoaded {
                    cid,
                    permission_graph: shared::demo_graph(),
                    peerset_address,
                })
                .await
                .unwrap();
        });
    }

    fn async_save_permission_graph(
        &self,
        _permission_graph: PermissionGraph,
        context: CommandEvent,
    ) {
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            sender
                .send(IPFSEvent::PermissionGraphSaved {
                    cid: shared::demo_graph_cid(),
                    context,
                })
                .await
                .unwrap();
        });
    }
}

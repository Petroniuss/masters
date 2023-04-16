use crate::core::protocol::IPFSEvent;
use crate::errors::Result;
use crate::transport::grpc::command::PermissionGraph;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use prost::Message;
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub type CID = String;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String);

    /// Context is passed during asynchronous callback
    fn async_save_permission_graph(
        &self,
        permission_graph: PermissionGraph,
        peerset_address: Option<String>,
    );

    fn async_save_permission_graphs(
        &self,
        peerset_address: String,
        this_peerset_permission_graph: PermissionGraph,
        other_peerset_address: String,
        other_peerset_permission_graph: PermissionGraph,
    );
}

pub struct IPFSWrapper {
    pub ipfs_sender: Sender<IPFSEvent>,
    pub ipfs_client: Arc<IpfsClient>,
}

impl IPFSWrapper {
    pub fn new(ipfs_sender: Sender<IPFSEvent>) -> Self {
        let ipfs_client = IpfsClient::default();
        return Self::new_with_ipfs_client(ipfs_sender, ipfs_client);
    }

    pub fn new_with_ipfs_client(ipfs_sender: Sender<IPFSEvent>, ipfs_client: IpfsClient) -> Self {
        return Self {
            ipfs_sender,
            ipfs_client: Arc::new(ipfs_client),
        };
    }
}

impl IPFSFacade for IPFSWrapper {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String) {
        let ipfs_client = self.ipfs_client.clone();
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            let permission_graph = read_permission_graph(ipfs_client, &cid).await.unwrap();

            sender
                .send(IPFSEvent::PermissionGraphLoaded {
                    cid,
                    permission_graph,
                    peerset_address,
                })
                .await
                .unwrap();
        });
    }

    fn async_save_permission_graph(
        &self,
        permission_graph: PermissionGraph,
        peerset_address: Option<String>,
    ) {
        let ipfs_client = self.ipfs_client.clone();
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            let cid = save_permission_graph(&ipfs_client, permission_graph)
                .await
                .unwrap();

            sender
                .send(IPFSEvent::PermissionGraphSaved {
                    cid,
                    peerset_address,
                })
                .await
                .unwrap();
        });
    }

    fn async_save_permission_graphs(
        &self,
        peerset_address: String,
        this_peerset_permission_graph: PermissionGraph,
        other_peerset_address: String,
        other_peerset_permission_graph: PermissionGraph,
    ) {
        let ipfs_client = self.ipfs_client.clone();
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            let (this_cid_result, other_cid_result) = tokio::join!(
                save_permission_graph(&ipfs_client, this_peerset_permission_graph),
                save_permission_graph(&ipfs_client, other_peerset_permission_graph)
            );

            sender
                .send(IPFSEvent::CrossPeersetPermissionGraphsSaved {
                    peerset_address,
                    this_peerset_cid: this_cid_result.unwrap(),
                    other_peerset_address,
                    other_peerset_cid: other_cid_result.unwrap(),
                })
                .await
                .unwrap();
        });
    }
}

async fn save_permission_graph(
    ipfs_client: &Arc<IpfsClient>,
    permission_graph: PermissionGraph,
) -> Result<CID> {
    let bytes = PermissionGraph::encode_to_vec(&permission_graph);
    let buff = Cursor::new(bytes);

    let response = ipfs_client.add(buff).await?;
    let cid = response.hash;

    Ok(cid)
}

async fn read_permission_graph(ipfs_client: Arc<IpfsClient>, cid: &CID) -> Result<PermissionGraph> {
    let bytes = ipfs_client
        .cat(cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await?;

    let permission_graph = PermissionGraph::decode(bytes.as_ref())?;

    Ok(permission_graph)
}

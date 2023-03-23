use crate::errors::Result;
use crate::grpc::command::PermissionGraph;
use prost::Message;

pub struct IPFSClientFacade {}

pub type CID = String;

impl IPFSClientFacade {
    pub async fn upload_permission_graph(&self, permission_graph: PermissionGraph) -> Result<CID> {
        let mut buf = vec![];
        permission_graph.encode(&mut buf)?;

        let cid = "ipfs://test-cid";
        // todo: integrate with ipfs

        Ok(cid.to_string())
    }

    pub async fn download_permission_graph(&self) -> Result<String> {
        panic!("Not implemented yet")
    }
}

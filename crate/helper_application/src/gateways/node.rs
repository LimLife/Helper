use crate::gateways::ContentError;
use domain::{data::NodeMeta, id::NodeMetaID};
#[allow(async_fn_in_trait)]
pub trait NodeGateway {
    async fn load_root(&self) -> Result<Vec<NodeMeta>, ContentError>;
    async fn load_children(&self, parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, ContentError>;
}

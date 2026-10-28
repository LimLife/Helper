use domain::{data::NodeMeta, id::NodeMetaID};

#[allow(async_fn_in_trait)]
pub trait NodeRepository {
    async fn root(&self) -> Result<Vec<NodeMeta>, String>;
    async fn children(&self, parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, String>;
}

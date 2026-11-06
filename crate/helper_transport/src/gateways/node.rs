use crate::dioxus::node::{load_children, load_root};
use application::gateway::{ContentError, NodeGateway};
use domain::data::NodeMeta;
#[derive(Default, Clone, Copy)]
pub struct NodeGatewayDioxus;

impl NodeGateway for NodeGatewayDioxus {
    async fn load_root(&self) -> Result<Vec<NodeMeta>, ContentError> {
        load_root()
            .await
            .map_err(|err| ContentError::Transport(err.to_string()))
    }

    async fn load_children(
        &self,
        parent_id: domain::id::NodeMetaID,
    ) -> Result<Vec<NodeMeta>, ContentError> {
        load_children(parent_id)
            .await
            .map_err(|err| ContentError::Transport(err.to_string()))
    }
}

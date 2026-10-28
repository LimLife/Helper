#[cfg(feature = "server")]
use crate::state::Repository;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use domain::{data::NodeMeta, id::NodeMetaID};

#[server(service: Extension <Repository>)]
pub async fn load_root() -> Result<Vec<NodeMeta>, ServerFnError> {
    service
        .0
        .content_service
        .load_root()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

#[server(service: Extension <Repository>)]
pub async fn load_children(parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, ServerFnError> {
    service
        .0
        .content_service
        .load_children(parent_id)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

use dioxus::prelude::*;
use shared::{
    manifesto::{Article, ArticleBody, NodeMeta},
    new_type_id::{ArticleID, NodeMetaID},
};

#[cfg(feature = "server")]
use shared::store::{ArticleStore, SectionStore};

#[server]
pub async fn server_load_children(parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, ServerFnError> {
    let (section, _): (SectionStore, ArticleStore) =
        crate::data_component::fake_data::data::fake_data();

    let children = section
        .children
        .get(&parent_id)
        .cloned()
        .unwrap_or_default();
    let nodes = children
        .into_iter()
        .filter_map(|id| section.nodes.get(&id).cloned())
        .collect();
    Ok(nodes)
}
#[server]
pub async fn server_load_article(article_id: ArticleID) -> Result<Article, ServerFnError> {
    let (_, article_store) = crate::data_component::fake_data::data::fake_data();
    article_store
        .articles
        .get(&article_id)
        .cloned()
        .ok_or(ServerFnError::new("Article not found"))
}

#[server]
pub async fn server_load_article_body(article_id: ArticleID) -> Result<ArticleBody, ServerFnError> {
    let (_, article_store) = crate::data_component::fake_data::data::fake_data();
    article_store
        .bodies
        .get(&article_id)
        .cloned()
        .ok_or(ServerFnError::new("Body not found"))
}

#[server]
pub async fn server_load_root() -> Result<Vec<NodeMeta>, ServerFnError> {
    let s = vec![NodeMeta::new()];
    Ok(s)
}

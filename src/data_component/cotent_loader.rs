use shared::{
    manifesto::{Article, ArticleBody, NodeMeta},
    new_type_id::{ArticleID, NodeMetaID},
};

use crate::data_component::server_loader::{
    server_load_article, server_load_article_body, server_load_children, server_load_root,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_children(&self, parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, String> {
        server_load_children(parent_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn load_article(&self, article_id: ArticleID) -> Result<Article, String> {
        server_load_article(article_id)
            .await
            .map_err(|e| e.to_string())
    }
    pub async fn load_root(&self) -> Result<Vec<NodeMeta>, String> {
        server_load_root().await.map_err(|e| e.to_string())
    }
    #[allow(dead_code)]
    pub async fn load_article_body(&self, article_id: ArticleID) -> Result<ArticleBody, String> {
        server_load_article_body(article_id)
            .await
            .map_err(|e| e.to_string())
    }
}

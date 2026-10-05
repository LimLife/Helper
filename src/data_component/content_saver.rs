use crate::data_component::server_saver::*;
use shared::{
    manifesto::Content,
    new_type_id::{ArticleID, ContentID},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Saver;

#[allow(dead_code)]
impl Saver {
    pub fn new() -> Self {
        Self
    }
    pub async fn create_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<Content, String> {
        server_create_content(article_id, content)
            .await
            .map_err(|e| e.to_string())
    }
    pub async fn update_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<Content, String> {
        server_update_content(article_id, content)
            .await
            .map_err(|e| e.to_string())
    }
    pub async fn delete_content(
        &self,
        article_id: ArticleID,
        content_id: ContentID,
    ) -> Result<(), String> {
        server_delete_content(article_id, content_id)
            .await
            .map_err(|e| e.to_string())
    }
}

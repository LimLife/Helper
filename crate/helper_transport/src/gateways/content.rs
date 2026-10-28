use domain::{
    data::Content,
    id::{ArticleID, ContentID},
};

use crate::dioxus::content::{create_content_server, delete_content_server, update_content_server};
use application::gateway::{ContentError, ContentGateway};
#[derive(Clone, Copy)]
pub struct ContentGatewayDioxus;

impl ContentGateway for ContentGatewayDioxus {
    async fn create_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, ContentError> {
        create_content_server(article_id, content)
            .await
            .map_err(|err| ContentError::Transport(err.to_string()))
    }

    async fn update_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, ContentError> {
        update_content_server(article_id, content)
            .await
            .map_err(|err| ContentError::Transport(err.to_string()))
    }

    async fn delete_content(
        &self,
        article_id: domain::id::ArticleID,
        content_id: domain::id::ContentID,
    ) -> Result<(), ContentError> {
        delete_content_server(article_id, content_id)
            .await
            .map_err(|err| ContentError::Transport(err.to_string()))
    }
}

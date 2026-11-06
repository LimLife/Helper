use crate::dioxus::article::{article, article_body};
use application::gateway::ArticleGateway;

#[derive(Default, Clone, Copy)]
pub struct ArticleGatewayDioxus;

impl ArticleGateway for ArticleGatewayDioxus {
    async fn article(
        &self,
        article_id: domain::id::ArticleID,
    ) -> Result<domain::data::Article, String> {
        article(article_id).await.map_err(|err| err.to_string())
    }

    async fn article_body(
        &self,
        article_id: domain::id::ArticleID,
    ) -> Result<domain::data::ArticleBody, String> {
        article_body(article_id)
            .await
            .map_err(|err| err.to_string())
    }
}

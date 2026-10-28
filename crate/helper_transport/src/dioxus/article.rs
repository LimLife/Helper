#[cfg(feature = "server")]
use crate::state::Repository;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use domain::data::{Article, ArticleBody};
use domain::id::ArticleID;

#[server(db:Extension<Repository>)]
pub async fn article(parent_id: ArticleID) -> Result<Article, ServerFnError> {
    db.0.content_service
        .article(parent_id)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}
#[server(db:Extension<Repository>)]
pub async fn article_body(article_id: ArticleID) -> Result<ArticleBody, ServerFnError> {
    db.0.content_service
        .article_body(article_id)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

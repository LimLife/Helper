#[cfg(feature = "server")]
use crate::state::Repository;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use domain::data::Content;
use domain::id::ArticleID;
//#[cfg(feature = "server")]
use domain::id::ContentID;
#[server(db:Extension<Repository>)]
pub async fn create_content_server(
    article_id: ArticleID,
    content: Content,
) -> Result<ContentID, ServerFnError> {
    let service = db.0;
    service
        .content_service
        .create_content(article_id, content)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

#[server(db:Extension<Repository>)]
pub async fn update_content_server(
    article_id: ArticleID,
    content: Content,
) -> Result<ContentID, ServerFnError> {
    let service = db.0;
    service
        .content_service
        .update_content(article_id, content)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

#[server(db:Extension<Repository>)]
pub async fn delete_content_server(
    article_id: ArticleID,
    content_id: ContentID,
) -> Result<(), ServerFnError> {
    let service = db.0;
    service
        .content_service
        .delete_content(article_id, content_id)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

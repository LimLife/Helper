use dioxus::prelude::*;
use domain::{data::Content, id::ArticleID};

#[server]
pub async fn server_update_content(
    article_id: ArticleID,
    content: Content,
) -> Result<Content, ServerFnError> {
    todo!()
}

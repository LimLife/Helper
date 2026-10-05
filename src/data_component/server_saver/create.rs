use dioxus::prelude::*;
use shared::{manifesto::Content, new_type_id::ArticleID};

#[server]
pub async fn server_create_content(
    article_id: ArticleID,
    content: Content,
) -> Result<Content, ServerFnError> {
    todo!()
}

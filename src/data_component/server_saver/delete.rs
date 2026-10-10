use dioxus::prelude::*;
use domain::id::{ArticleID, ContentID};

#[server]
pub async fn server_delete_content(
    article_id: ArticleID,
    content_id: ContentID,
) -> Result<(), ServerFnError> {
    todo!()
}

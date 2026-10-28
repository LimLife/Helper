use domain::{
    data::Content,
    id::{ArticleID, ContentID},
};

use crate::gateways::ContentError;

#[allow(async_fn_in_trait)]
pub trait ContentGateway {
    async fn create_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, ContentError>;

    async fn update_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, ContentError>;

    async fn delete_content(
        &self,
        article_id: ArticleID,
        content_id: ContentID,
    ) -> Result<(), ContentError>;
}

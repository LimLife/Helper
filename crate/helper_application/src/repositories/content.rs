use domain::{
    data::Content,
    id::{ArticleID, ContentID},
};

#[allow(async_fn_in_trait)]
pub trait ContentRepository {
    async fn create(&self, article_id: ArticleID, content: Content) -> Result<ContentID, String>;
    async fn update(&self, article_id: ArticleID, content: Content) -> Result<ContentID, String>;
    async fn delete(&self, article_id: ArticleID, content_id: ContentID) -> Result<(), String>;
}

use domain::{
    data::{Article, ArticleBody},
    id::ArticleID,
};

#[allow(async_fn_in_trait)]
pub trait ArticleGateway {
    async fn article(&self, article_id: ArticleID) -> Result<Article, String>;
    async fn article_body(&self, article_id: ArticleID) -> Result<ArticleBody, String>;
}

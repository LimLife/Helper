use domain::{data::Article, id::ArticleID};

#[allow(async_fn_in_trait)]
pub trait ArticleBodyRepository {
    async fn article(&self, article_id: ArticleID) -> Result<Article, String>;
}

use domain::{
    data::{Article, ArticleBody, Content, NodeMeta},
    id::{ArticleID, ContentID, NodeMetaID},
};

use crate::repositories::{
    article::ArticleRepository, content::ContentRepository, node::NodeRepository,
};

#[derive(Debug, Clone)]
pub struct ContentService<C, N, A> {
    content: C,
    node: N,
    article: A,
}

impl<C, N, A> ContentService<C, N, A> {
    pub fn new(content: C, node: N, article: A) -> Self {
        Self {
            content,
            node,
            article,
        }
    }
}
impl<C, N, A> ContentService<C, N, A>
where
    C: ContentRepository + Clone + Send + Sync + 'static,
    N: NodeRepository + Clone + Send + Sync + 'static,
    A: ArticleRepository + Clone + Send + Sync + 'static,
{
    pub async fn create_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, String> {
        self.content.create(article_id, content).await
    }
    pub async fn update_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, String> {
        self.content.update(article_id, content).await
    }
    pub async fn delete_content(
        &self,
        article_id: ArticleID,
        content_id: ContentID,
    ) -> Result<(), String> {
        self.content.delete(article_id, content_id).await
    }

    pub async fn load_root(&self) -> Result<Vec<NodeMeta>, String> {
        self.node.root().await
    }
    pub async fn load_children(&self, parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, String> {
        self.node.children(parent_id).await
    }

    pub async fn article(&self, article_id: ArticleID) -> Result<Article, String> {
        self.article.article(article_id).await
    }
    pub async fn article_body(&self, article_id: ArticleID) -> Result<ArticleBody, String> {
        self.article.article_body(article_id).await
    }
}

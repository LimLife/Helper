use core::fmt;

use crate::gateway::{ArticleGateway, ContentGateway, NodeGateway};
use domain::{
    data::{Article, ArticleBody, Content, NodeMeta},
    id::{ArticleID, ContentID, NodeMetaID},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Loader<C: ContentGateway, N: NodeGateway, A: ArticleGateway> {
    content: C,
    node: N,
    article: A,
}
#[derive(Default, Debug, Clone, Copy)]
pub struct Mutator<C: ContentGateway, N: NodeGateway, A: ArticleGateway> {
    content: C,
    node: N,
    article: A,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    BadRequest(String),
}
impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::BadRequest(msg) => write!(f, "{:?}", msg),
        }
    }
}

impl<C: ContentGateway, N: NodeGateway, A: ArticleGateway> Loader<C, N, A> {
    pub fn new(content: C, node: N, article: A) -> Self {
        Self {
            content,
            node,
            article,
        }
    }
    pub async fn load_children(&self, parent_id: NodeMetaID) -> Result<Vec<NodeMeta>, LoadError> {
        self.node
            .load_children(parent_id)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
    pub async fn load_root(&self) -> Result<Vec<NodeMeta>, LoadError> {
        self.node
            .load_root()
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
    pub async fn article(&self, article_id: ArticleID) -> Result<Article, LoadError> {
        self.article
            .article(article_id)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
    pub async fn load_article_body(&self, article_id: ArticleID) -> Result<ArticleBody, LoadError> {
        self.article
            .article_body(article_id)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
}
impl<C: ContentGateway, N: NodeGateway, A: ArticleGateway> Mutator<C, N, A> {
    pub fn new(content: C, node: N, article: A) -> Self {
        Self {
            content,
            node,
            article,
        }
    }
    pub async fn create_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, LoadError> {
        self.content
            .create_content(article_id, content)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
    pub async fn update_content(
        &self,
        article_id: ArticleID,
        content: Content,
    ) -> Result<ContentID, LoadError> {
        self.content
            .update_content(article_id, content)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }

    pub async fn delete_content(
        &self,
        article_id: ArticleID,
        content_id: ContentID,
    ) -> Result<(), LoadError> {
        self.content
            .delete_content(article_id, content_id)
            .await
            .map_err(|err| LoadError::BadRequest(err.to_string()))
    }
}

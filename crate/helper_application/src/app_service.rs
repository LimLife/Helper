use crate::{
    repositories::{article::ArticleRepository, content::ContentRepository, node::NodeRepository},
    service::ContentService,
};

#[derive(Debug, Clone)]
pub struct AppService<C, N, A>
where
    C: ContentRepository + Clone + Send + Sync + 'static,
    N: NodeRepository + Clone + Send + Sync + 'static,
    A: ArticleRepository + Clone + Send + Sync + 'static,
{
    pub content_service: ContentService<C, N, A>,
}

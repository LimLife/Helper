use application::app_service::AppService;
use application::config::DBConfig;
use application::service::ContentService;
#[cfg(feature = "server")]
pub type Repository =
    AppService<PostgresContentRepository, PostgresNodeRepository, PostgresArticleRepository>;
#[cfg(feature = "server")]
use infrastructure::postgres::{
    article::PostgresArticleRepository, content::PostgresContentRepository,
    node::PostgresNodeRepository, pool::create_pool,
};
#[cfg(feature = "server")]
use infrastructure::sql::{migrate_database, seed_database};
#[cfg(feature = "server")]
pub async fn create_app_state(config: DBConfig) -> Result<Repository, anyhow::Error> {
    let seed = config.seed_mode.clone();
    let pool = create_pool(config).await?;
    migrate_database(pool.clone()).await?;
    seed_database(&pool, seed).await?;
    let node = PostgresNodeRepository::new(pool.clone());
    let content = PostgresContentRepository::new(pool.clone());
    let article = PostgresArticleRepository::new(pool.clone());
    let content_service = ContentService::new(content, node, article);
    let service = AppService { content_service };
    Ok(service)
}

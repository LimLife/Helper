use application::config::DBConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(config: DBConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.database_max_connection)
        .connect(&config.database_url)
        .await
}

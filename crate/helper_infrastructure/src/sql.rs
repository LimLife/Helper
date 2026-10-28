use application::config::SeedMode;
use sqlx::PgPool;

pub(crate) mod migrations;
pub(crate) mod seeds;

pub async fn migrate_database(pool: PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./src/sql/migrations").run(&pool).await?;
    //sqlx::migrate::Migrator::new(Path::new(".sql/migrations")).await?;
    Ok(())
}

pub async fn seed_database(pool: &PgPool, seed: SeedMode) -> Result<(), sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM articles")
        .fetch_one(&pool.clone())
        .await?;
    if count == 0 {
        Ok(())
    } else {
        let seed_sql: Option<&str> = match seed {
            SeedMode::None => None,
            SeedMode::Dev => Some(include_str!("./sql/seeds/development.sql")),
            SeedMode::Demo => None,
            SeedMode::Test => None,
        };
        match seed_sql {
            Some(sql_raw) => {
                sqlx::raw_sql(sql_raw).execute(pool).await?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

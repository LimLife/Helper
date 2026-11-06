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
    let Some(script) = seed_script(seed) else {
        return Ok(());
    };

    let mut tx = pool.begin().await?;
    sqlx::raw_sql(script).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
fn seed_script(mode: SeedMode) -> Option<&'static str> {
    match mode {
        SeedMode::None => None,
        SeedMode::Dev => Some(include_str!("./sql/seeds/development.sql")),
        SeedMode::Demo => None,
        SeedMode::Test => None,
    }
}

use application::config::{DBConfig, SeedMode};
use std::str::FromStr;

pub fn load() -> Result<DBConfig, String> {
    let database_max_connection = std::env::var("DATABASE_MAX_CONNECTIONS")
        .map_err(|_| "DATABASE_MAX_CONNECTIONS must be set".to_string())?
        .parse::<u32>()
        .map_err(|err| format!("DATABASE_MAX_CONNECTIONS is invalid: {err}"))?;

    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;

    let seed_mode = std::env::var("SEED_MODE")
        .map_err(|_| "SEED_MODE must be set".to_string())
        .and_then(|value| SeedMode::from_str(&value))?;

    Ok(DBConfig::new(
        database_url,
        database_max_connection,
        seed_mode,
    ))
}

#[derive(Debug, Clone)]
pub struct DBConfig {
    pub database_url: String,
    pub database_max_connection: u32,
    pub seed_mode: SeedMode,
}

impl DBConfig {
    pub fn new(database_url: String, database_max_connection: u32, seed_mode: SeedMode) -> Self {
        Self {
            database_url,
            database_max_connection,
            seed_mode,
        }
    }
}
#[derive(Debug, Clone)]
pub enum SeedMode {
    None,
    Dev,
    Demo,
    Test,
}

impl std::str::FromStr for SeedMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Development" => Ok(SeedMode::Dev),
            "Demo" => Ok(SeedMode::Demo),
            "Test" => Ok(SeedMode::Test),
            "" => Ok(SeedMode::None),
            other => Err(format!("unknown SEED_MODE: {other:?}")),
        }
    }
}

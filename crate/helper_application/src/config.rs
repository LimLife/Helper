#[derive(Debug, Clone)]
pub struct DBConfig {
    pub database_url: String,
    pub database_max_connection: u32,
    pub seed_mode: SeedMode,
}

#[derive(Debug, Clone)]
pub enum SeedMode {
    None,
    Dev,
    Demo,
    Test,
}

impl std::str::FromStr for SeedMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Development" => Result::Ok(SeedMode::Dev),
            "Demo" => Result::Ok(SeedMode::Demo),
            "Test" => Result::Ok(SeedMode::Test),
            s if s.is_empty() => Result::Ok(SeedMode::None),
            _ => Result::Ok(SeedMode::None),
        }
    }
}

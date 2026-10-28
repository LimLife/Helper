use crate::app::App;

mod app;
mod components;
mod data_use_cases;
mod pages;
mod render_components;

#[cfg(feature = "server")]
use std::str::FromStr;

#[cfg(feature = "server")]
use application::config::{DBConfig, SeedMode};

#[cfg(feature = "server")]
use dioxus::server::{axum, DioxusRouterExt};

#[cfg(feature = "server")]
use transport::state::create_app_state;

fn main() {
    #[cfg(feature = "server")]
    {
        dioxus::serve(|| async move {

            use dioxus::server::axum::Extension;

            dotenvy::dotenv().ok();

            let config = DBConfig {
                database_max_connection: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .expect("DATABASE_MAX_CONNECTIONS must be set")
                    .parse()?,

                database_url: std::env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set")
                    .parse()?,

                seed_mode: SeedMode::from_str(std::env::var("SEED_MODE").as_deref().unwrap_or(""))
                    .unwrap(),
            };

            let server_config = dioxus::server::ServeConfig::default();
            let state = create_app_state(config).await?;

            Ok(axum::Router::new()
                .serve_dioxus_application(server_config, App)
                .layer(Extension(state)))
        });
    }

    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }
}

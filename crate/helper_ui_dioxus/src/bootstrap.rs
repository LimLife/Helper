use crate::app::App;

#[cfg(feature = "server")]
use application::config::DBConfig;

#[cfg(feature = "server")]
use dioxus::server::{DioxusRouterExt, axum};

#[cfg(feature = "server")]
use transport::state::create_app_state;

#[cfg(feature = "server")]
pub fn bootstrap_server(config: DBConfig) {
    dioxus::serve(move || {
        let config = config.clone();

        async move {
            use dioxus::server::axum::Extension;

            let server_config = dioxus::server::ServeConfig::default();
            let state = create_app_state(config).await?;

            Ok(axum::Router::new()
                .serve_dioxus_application(server_config, App)
                .layer(Extension(state)))
        }
    });
}

#[cfg(all(feature = "web", not(feature = "server")))]
pub fn bootstrap_client() {
    use dioxus_logger::tracing::Level;

    dioxus_logger::init(Level::INFO).expect("Logger initialization failed");

    dioxus::launch(App);
}

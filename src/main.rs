mod config;

#[cfg(feature = "server")]
use crate::config::load;

#[cfg(feature = "server")]
use dioxus_ui::bootstrap::bootstrap_server;

#[cfg(all(feature = "web", not(feature = "server")))]
use dioxus_ui::bootstrap::bootstrap_client;

fn main() {
    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        console_error_panic_hook::set_once();
        bootstrap_client();
    }

    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    {
        dotenvy::dotenv().ok();

        let config = load().expect("Failed to load server configuration");

        bootstrap_server(config);
    }
}

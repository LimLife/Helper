mod app;
mod components;
use crate::components::elements::app::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let pool = create_pg_pool().await;
    LaunchBuilder::new()
        .with_state(AppState { db: pool })
        .with_cfg(server_only!(
            dioxus_server::ServeConfig::default()
        ))
        .launch(App);
}
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
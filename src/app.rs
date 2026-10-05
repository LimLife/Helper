use dioxus::prelude::*;
use crate::components::elements::app::App;

pub fn set_app() {
    #[cfg(feature = "server")] 
        LaunchBuilder::new()
        .with_cfg(server_only!(
            dioxus_server::ServeConfig::default()
            .incremental(dioxus_server::IncrementalRendererConfig::default())))
        .launch(App);
    
     dioxus::launch(App);
}


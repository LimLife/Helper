use dioxus::prelude::*;

use crate::components::{assets, routes::main_route::Route};
#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: assets::images::FAVICON }
        document::Link { rel: "stylesheet", href: assets::css::MAIN_CSS }
        Router::<Route> {}
    }
}
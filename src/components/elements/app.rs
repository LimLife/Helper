use dioxus::prelude::*;

use crate::components::{assets::{
    css::import_styles,
    images::FAVICON
},
    state::{
        navigation::nav_navigation_store::NavigationStore
    },
    routes::main_route::Route
};
#[component]
pub fn App() -> Element {
    let nav_store = use_signal(|| NavigationStore::default());
    use_context_provider(|| nav_store);
    rsx! {
        import_styles {}

        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
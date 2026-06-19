use dioxus::prelude::*;

use crate::components::{
    assets::{css::import_styles, images::FAVICON},
    routes::main_route::Route,
    state::navigation::nav_navigation_store::NavigationStore,
};
use crate::data_component::fake_data::data::fake_section_store;
#[component]
pub fn App() -> Element {
    let nav_store = use_signal(|| NavigationStore::default());
    use_context_provider(|| nav_store);

    let store_meta = use_signal(|| fake_section_store());
    use_context_provider(|| store_meta);
    rsx! {
        import_styles {}

        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

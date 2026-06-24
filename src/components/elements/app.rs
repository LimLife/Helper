use dioxus::prelude::*;

use crate::components::{
    assets::{css::import_styles, images::FAVICON},
    routes::main_route::Route,
    state::navigation::nav_navigation_store::NavigationStore,
};
use crate::data_component::{cotent_loader::Loader, fake_data::data::fake_data};

#[component]
pub fn App() -> Element {
    let nav_store = use_signal(|| NavigationStore::default());
    use_context_provider(|| nav_store);
    use_context_provider(|| Loader::new());
    let (section_data, article_data) = fake_data();

    let store_section = use_signal(|| section_data);
    let store_article = use_signal(|| article_data);
    use_context_provider(|| store_section);
    use_context_provider(|| store_article);
    rsx! {
        import_styles {}

        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

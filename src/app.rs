use crate::components::{
    assets::{css::import_styles, images::FAVICON},
    routes::main_route::Route,
    state::navigation::nav_navigation_store::NavigationStore,
};
use crate::data_component::{
    content_saver::Saver, cotent_loader::Loader, fake_data::data::fake_data,
};

use dioxus::prelude::*;
use shared::store::EditorStore;
#[component]
pub fn App() -> Element {
    let editor_store = use_signal(|| EditorStore::default());
    let nav_store = use_signal(|| NavigationStore::default());
    //let loader = use_signal(|| Loader::new());
    let saver = use_signal(|| Saver::new());
    use_context_provider(|| nav_store);
    use_context_provider(|| Loader::new());
    use_context_provider(|| saver);
    let (section_data, article_data) = fake_data();
    let store_section = use_signal(|| section_data);
    let store_article = use_signal(|| article_data);
    use_context_provider(|| store_section);
    use_context_provider(|| store_article);
    use_context_provider(|| editor_store);

    rsx! {
        import_styles {}
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

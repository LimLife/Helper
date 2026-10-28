use crate::{
    components::{
        assets::{css::import_styles, images::FAVICON},
        routes::main_route::Route,
        state::navigation::nav_navigation_store::NavigationStore,
    },
    data_use_cases::{AppLoader, AppMutator},
};

use application::store::{ArticleStore, EditorStore, SectionStore};
use dioxus::prelude::*;

use transport::gateways::{
    article::ArticleGatewayDioxus, content::ContentGatewayDioxus, node::NodeGatewayDioxus,
};

#[component]
pub fn App() -> Element {
    let editor_store = use_signal(|| EditorStore::default());
    let nav_store = use_signal(|| NavigationStore::default());
    let loader = use_signal(|| {
        AppLoader::new(
            ContentGatewayDioxus,
            NodeGatewayDioxus,
            ArticleGatewayDioxus,
        )
    });
    let mutator = use_signal(|| {
        AppMutator::new(
            ContentGatewayDioxus,
            NodeGatewayDioxus,
            ArticleGatewayDioxus,
        )
    });
    let article_store = ArticleStore::default();
    let section_store = SectionStore::default();
    use_context_provider(|| nav_store);
    use_context_provider(|| mutator);
    use_context_provider(|| loader);
    let section = use_signal(|| article_store);
    let article = use_signal(|| section_store);
    use_context_provider(|| section);
    use_context_provider(|| article);
    use_context_provider(|| editor_store);

    rsx! {
        import_styles {}
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

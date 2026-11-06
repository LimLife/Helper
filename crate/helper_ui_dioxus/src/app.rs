use crate::{
    assets::{css::import_styles, images::FAVICON},
    routes::Route,
};

use crate::types::{AppLoader, AppMutator};
use application::store::{ArticleStore, EditorStore, SectionStore};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let editor_store = use_signal(|| EditorStore::default());

    let loader = AppLoader::default();
    let mutator = AppMutator::default();

    let article_store = ArticleStore::default();
    let section_store = SectionStore::default();
    let article = use_signal(|| article_store);
    let section = use_signal(|| section_store);
    use_context_provider(|| mutator);
    use_context_provider(|| loader);
    use_context_provider(|| section);
    use_context_provider(|| article);
    use_context_provider(|| editor_store);

    rsx! {
        import_styles {}
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

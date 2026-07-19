use dioxus::prelude::*;

use crate::components::elements::shared_elements::tree_root::TreeRoot;

use crate::render_components::editor_render::RenderCoomponent;
use shared::store::ArticleStore;
#[component]
pub fn AdminPage() -> Element {
    let article_store = use_context::<Signal<ArticleStore>>();
    let blocks = {
        let store = article_store.read();
        store
            .current_body()
            .map(|body| body.bloks.clone())
            .unwrap_or_default()
    };
    rsx! {
        div {
            id: "admin-page",
            class: "admin-page",
        TreeRoot {  }
        RenderCoomponent { node_block:blocks }
        }
    }
}

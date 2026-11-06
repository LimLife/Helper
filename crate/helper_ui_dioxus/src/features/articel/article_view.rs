use application::store::ArticleStore;
use dioxus::prelude::*;

use crate::components::rendering::content_render::RenderComponent;

#[component]
pub fn ArticleView() -> Element {
    let article_store = use_context::<Signal<ArticleStore>>();

    let blocks = {
        let store = article_store.read();

        store
            .current_body()
            .map(|body| body.blocks.clone())
            .unwrap_or_default()
    };

    rsx! {
        RenderComponent {
            node_block: blocks
        }
    }
}

mod content_area;
pub mod sidebar_left;
pub mod sidebar_right;
use crate::components::elements::shared_elements::tree_root;
use dioxus::prelude::*;

use super::main_layout::{
    content_area::ContentArea, sidebar_left::SidebarLeft, sidebar_right::SidebarRight,
};
use crate::render_components::content_render::RenderCoomponent;
use shared::{manifesto::NodeMeta, store::ArticleStore};
#[component]
pub fn MainLayout() -> Element {
    let node_right = use_signal(|| vec![NodeMeta::new()]);
    let article_store = use_context::<Signal<ArticleStore>>();
    let blocks = {
        let store = article_store.read();
        store
            .current_body()
            .map(|body| body.bloks.clone())
            .unwrap_or_default()
    };
    rsx! {
        main { id: "main-layout", class: "main-layout",
            SidebarLeft {},
            ContentArea {
                RenderCoomponent{node_block: blocks}
            }
            SidebarRight {node_right}
        }
    }
}

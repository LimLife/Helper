mod content_area;
mod sidebar_left;
mod sidebar_right;
mod tree_root;
use dioxus::prelude::*;

use super::main_layout::{
    content_area::ContentArea, sidebar_left::SidebarLeft, sidebar_right::SidebarRight,
};
use crate::render_components::content_render::RenderCoomponent;
use shared::{
    data_block::CodeData,
    manifesto::{Block, NodeMeta},
    store::ArticleStore,
};
#[component]
pub fn MainLayout() -> Element {
    //сдесь должны быть разные масивы и должен быть
    let node_left = use_signal(|| vec![NodeMeta::new()]);
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
            SidebarLeft {node_left },
            ContentArea {
                RenderCoomponent{node_block: blocks}
            }
            SidebarRight {node_right}
        }
    }
}

use dioxus::prelude::*;

use crate::components::elements::shared_elements::tree_root::TreeRoot;

use crate::render_components::editor_render::RenderCoomponent;
use shared::manifesto::NodeMeta;
use shared::store::ArticleStore;
//temp
use crate::components::elements::layouts::main_layout::sidebar_right::SidebarRight;
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
    let node_right = use_signal(|| vec![NodeMeta::new()]);
    rsx! {
        div {
            id: "admin-page",
            class: "edior-layout",
            div {
                id: "editor-root-tree",
                TreeRoot {  },
            }
            div {
                id: "editor-render",
                RenderCoomponent { node_block:blocks }
            }
            div {
                id: "editor-right-bar",
                SidebarRight{ node_right }
            }
        }
    }
}

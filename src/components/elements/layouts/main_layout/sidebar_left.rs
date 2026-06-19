mod language_tree;
mod recent_section;
mod sidebar_header;
pub mod tree;

use super::sidebar_left::sidebar_header::SidebarHeader;
use super::tree_root::TreeRoot;
use dioxus::prelude::*;
use shared::manifesto::NodeMeta;
#[component]
pub fn SidebarLeft(node_left: Signal<Vec<NodeMeta>>) -> Element {
    rsx! {
        div { id: "left-sidebar", class: "left-sidebar",
            SidebarHeader {}
            TreeRoot {  }
        }
    }
}

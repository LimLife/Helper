mod language_tree;
mod recent_section;
mod sidebar_header;

use super::sidebar_left::{
    language_tree::LanguageTree, recent_section::RecentSection, sidebar_header::SidebarHeader,
};
use dioxus::prelude::*;
use shared::manifesto::NodeMeta;

#[component]
pub fn SidebarLeft(node_left: Signal<Vec<NodeMeta>>) -> Element {
    rsx! {
        div { id: "left-sidebar", class: "left-sidebar",
            SidebarHeader {}
            LanguageTree {}
            RecentSection {}
        }
    }
}

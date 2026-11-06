//mod language_tree;
mod recent_section;
mod sidebar_header;
use crate::features::navigation::tree_root::TreeRoot;

use super::sidebar_left::sidebar_header::SidebarHeader;

use dioxus::prelude::*;
#[component]
pub fn SidebarLeft() -> Element {
    rsx! {
        div { id: "left-sidebar", class: "left-sidebar",
            SidebarHeader {}
            TreeRoot {}
        }
    }
}

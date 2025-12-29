mod language_tree;
mod recent_section;
mod sidebar_header;


use dioxus::prelude::*;

use super::sidebar_left::{
    language_tree::LanguageTree,
    recent_section::RecentSection,
    sidebar_header::SidebarHeader
};

#[component]
pub fn SidebarLeft() -> Element {
    rsx!{
        LanguageTree { },
        RecentSection { },
        SidebarHeader { }
    }
}
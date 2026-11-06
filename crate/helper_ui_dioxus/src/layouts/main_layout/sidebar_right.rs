mod discussion_preview;
mod display_settings;
mod external_resources;
mod page_tools;
mod table_of_contents;
use crate::features::articel::article_aside::ArticleAside;

pub mod aside_rightbar {
    pub use crate::layouts::main_layout::sidebar_right::{
        discussion_preview::DiscussionPreview, display_settings::DisplaySettings,
        external_resources::ExternalResources, page_tools::PageTools,
        table_of_contents::TableOfContents,
    };
}

use dioxus::prelude::*;

#[component]
pub fn SidebarRight() -> Element {
    rsx! {
        div { id: "sidebar-right", class: "sidebar-right",
            ArticleAside {}
        }
    }
}

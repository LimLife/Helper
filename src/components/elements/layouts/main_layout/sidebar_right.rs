mod table_of_contents;
mod page_tools;
mod display_settings;
mod discussion_preview;
mod external_resources;
use dioxus::prelude::*;

use super::sidebar_right::{
    table_of_contents::TableOfContents,
    page_tools::PageTools,
    display_settings::DisplaySettings,
    discussion_preview::DiscussionPreview,
    external_resources::ExternalResources
};
#[component]
pub fn SidebarRight() -> Element {
    rsx! {
        div { id: "sidebar-right", class: "sidebar-right",
            TableOfContents {}
            PageTools {}
            DisplaySettings {}
            DiscussionPreview {}
            ExternalResources {}
        }
    }
}
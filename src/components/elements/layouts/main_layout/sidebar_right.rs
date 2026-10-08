mod discussion_preview;
mod display_settings;
mod external_resources;
mod page_tools;
mod table_of_contents;
use dioxus::prelude::*;

use super::sidebar_right::{
    discussion_preview::DiscussionPreview, display_settings::DisplaySettings,
    external_resources::ExternalResources, page_tools::PageTools,
    table_of_contents::TableOfContents,
};
use shared::manifesto::NodeMeta;

#[component]
pub fn SidebarRight(node_right: Signal<Vec<NodeMeta>>) -> Element {
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

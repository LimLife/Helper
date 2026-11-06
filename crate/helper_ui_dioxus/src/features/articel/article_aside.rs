use crate::layouts::main_layout::sidebar_right::aside_rightbar::{
    DiscussionPreview, DisplaySettings, ExternalResources, PageTools, TableOfContents,
};
use dioxus::prelude::*;
#[component]
pub fn ArticleAside() -> Element {
    rsx! {
        aside {
            TableOfContents {}
            PageTools {}
            DisplaySettings {}
            DiscussionPreview {}
            ExternalResources {}
        }
    }
}

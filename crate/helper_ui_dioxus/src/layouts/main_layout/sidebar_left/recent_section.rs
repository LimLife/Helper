mod recent_header;
mod recent_item;
use crate::components::collapse::Collapse;

use super::recent_section::{recent_header::RecentHeader, recent_item::RecentItem};
use dioxus::prelude::*;
#[component]
pub fn RecentSection() -> Element {
    rsx! {
        Collapse {
            id: "recent-sections",
            label: rsx! {
                RecentHeader {}
            },
            content: rsx! {
                RecentItem {}
            },
        }
    }
}

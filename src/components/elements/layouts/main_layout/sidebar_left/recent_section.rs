mod recent_header;
mod recent_item;
use super::recent_section::{recent_header::RecentHeader, recent_item::RecentItem};
use crate::components::elements::ui::collapse::Collapse;
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

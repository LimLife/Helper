use dioxus::prelude::*;


#[component]
pub fn RecentItem() -> Element {
    rsx!{
        div { class: "recent-section-item", "RecentItem" }
    }
}
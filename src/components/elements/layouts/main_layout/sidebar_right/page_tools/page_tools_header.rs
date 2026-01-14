use dioxus::prelude::*;


#[component]
pub fn PageToolsHeader() -> Element {
    rsx! {
        span { class: "page-tools-header", "Page Tools" }
    }
}
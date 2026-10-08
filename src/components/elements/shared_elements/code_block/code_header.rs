use dioxus::prelude::*;

#[component]
pub fn CodeHeader() -> Element {
    rsx! {
        h2 { class: "code-block-header", "Javascript" }
    }
}

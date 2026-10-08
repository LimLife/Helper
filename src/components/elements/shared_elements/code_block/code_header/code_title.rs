use dioxus::prelude::*;


#[component]
pub fn CodeTitle() -> Element {
    rsx! {
        h3 { class: "code-block-title", {"JavaScript"} }
    }
}
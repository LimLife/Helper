use dioxus::prelude::*;


#[component]
pub fn ArticleDescription() -> Element {
    rsx!{
        div { class: "artical-header-description", "Them type in Rust" }
    }
}
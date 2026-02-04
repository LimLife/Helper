use dioxus::prelude::*;


#[component]
pub fn ArticleDescription() -> Element {
    rsx!{
        p { class: "artical-header-description", "Them type in Rust" }
    }
}
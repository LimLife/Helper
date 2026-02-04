use dioxus::prelude::*;


#[component]
pub fn ArticleTitle() -> Element{
    rsx!{
        h2 { class: "artical-header-title", "Rust && Types" }
    }
}
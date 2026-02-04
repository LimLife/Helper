use dioxus::prelude::*;


#[component]
pub fn ArticleTitle() -> Element{
    rsx!{
        h1 { class: "artical-header-title", "Rust && Types" }
    }
}
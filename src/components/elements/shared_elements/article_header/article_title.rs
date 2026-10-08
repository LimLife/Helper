use dioxus::prelude::*;

#[component]
pub fn ArticleTitle(title: Option<String>) -> Element {
    rsx! {
        h1 { class: "artical-header-title", {title.unwrap_or_default()} }
    }
}

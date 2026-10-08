use dioxus::prelude::*;

#[component]
pub fn ArticleDescription(description: Option<String>) -> Element {
    rsx! {
        p { class: "artical-header-description", {description.unwrap_or_default()} }
    }
}

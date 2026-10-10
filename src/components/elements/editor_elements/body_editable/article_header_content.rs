use crate::components::elements::shared_elements::article_header::ArticleHeader;
use dioxus::prelude::*;
use domain::data::ArticleHeaderData;
#[component]
pub fn ArticleHeaderEditor(header: ArticleHeaderData) -> Element {
    rsx! {
        div { id: "article-header-editor", class: "article-header-editor",
            ArticleHeader { header }
        }
    }
}

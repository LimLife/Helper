use dioxus::prelude::*;
use domain::data::ArticleHeaderData;

use crate::features::articel::article_header::ArticleHeader;
#[component]
pub fn ArticleHeaderEditor(header: ArticleHeaderData) -> Element {
    rsx! {
        div { id: "article-header-editor", class: "article-header-editor",
            ArticleHeader { header }
        }
    }
}

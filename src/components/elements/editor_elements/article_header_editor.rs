use crate::components::elements::shared_elements::article_header::ArticleHeader;
use dioxus::prelude::*;
use shared::data_block::ArticleHeaderData;

#[component]
pub fn ArticleHeaderEditor(header: ArticleHeaderData) -> Element {
    rsx! {
        ArticleHeader { header }
    }
}

mod article_description;
mod article_title;
use super::article_header::{article_description::ArticleDescription, article_title::ArticleTitle};
use crate::components::elements::ui::version_selector::VersionSelector;
use crate::components::models::selector_version::selector::Selector;
use dioxus::prelude::*;
use shared::data_block::ArticleHeaderData;

#[component]
pub fn ArticleHeader(header: ArticleHeaderData) -> Element {
    rsx! {
        header { id: "artical-header", class: "artical-header",
            div { class: "artical-header-text",
                ArticleTitle {title:header.title}
                ArticleDescription {description: header.description}
            }
            VersionSelector { selector: Selector::empty_arr()  }
        }
    }
}

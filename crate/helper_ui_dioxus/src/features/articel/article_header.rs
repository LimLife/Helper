mod article_description;
mod article_title;
use dioxus::prelude::*;
use domain::data::ArticleHeaderData;

use crate::components::version_selector::VersionSelector;
use crate::features::articel::article_header::{
    article_description::ArticleDescription, article_title::ArticleTitle,
};

use crate::features::navigation::selector_version::selector::Selector;
#[component]
pub fn ArticleHeader(header: ArticleHeaderData) -> Element {
    rsx! {
        header { id: "artical-header", class: "artical-header",
            div { class: "artical-header-text",
                ArticleTitle { title: header.title }
                ArticleDescription { description: header.description }
            }
            VersionSelector { selector: Selector::empty_arr() }
        }
    }
}

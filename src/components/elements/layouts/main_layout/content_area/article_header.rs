mod article_description;
mod article_title;
mod version_switcher;


use dioxus::prelude::*;
use super::article_header::{
    article_description::ArticleDescription,
    article_title::ArticleTitle,
    version_switcher::VersionSwitcher
};

#[component]
pub fn ArticleHeader() -> Element {
    rsx!{
        ArticleDescription {  },
        ArticleTitle {},
        VersionSwitcher {}
    }
}
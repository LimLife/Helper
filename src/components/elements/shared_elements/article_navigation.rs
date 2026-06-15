mod next_article_link;
mod prev_article_link;

use dioxus::prelude::*;

use super::article_navigation::{
    next_article_link::NextArticleLink, prev_article_link::PrevArticleLink,
};

use shared::data_block::ArticleNavigationData;
#[component]
pub fn ArticleNavigation(navigate: ArticleNavigationData) -> Element {
    rsx! {
        nav { id: "nav-link", class: "article-navigation",
            NextArticleLink {next: navigate.next, next_name: navigate.next_name},
            PrevArticleLink {prev: navigate.prev, prev_name: navigate.prev_name}
        }
    }
}

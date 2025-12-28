mod next_article_link;
mod prev_article_link;

use dioxus::prelude::*;

use super::article_navigation::{
    next_article_link::NextArticleLink,
    prev_article_link::PrevArticleLink
};

#[component]
pub fn ArticleNavigation()->Element{
    rsx!{
        NextArticleLink {  },
        PrevArticleLink {  }
    }
}
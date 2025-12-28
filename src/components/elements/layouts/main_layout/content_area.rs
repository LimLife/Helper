mod breadcrumbs;
mod article_header;
mod article_body;
mod article_navigation;
use dioxus::prelude::*;

use super::content_area::{
    breadcrumbs::Breadcrumb,
    article_header::ArticleHeader,
    article_body::ArticleBody,
    article_navigation::ArticleNavigation
};

#[component]
pub fn ContentArea() -> Element {
    rsx!{
        Breadcrumb {  },
        ArticleHeader {},
        ArticleBody {},
        ArticleNavigation {}
    }
}
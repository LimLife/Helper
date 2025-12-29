mod toc_item;


use dioxus::prelude::*;
use super::table_of_contents::{
    toc_item::TocItem
};

#[component]
pub fn TableOfContents()->Element{
    rsx!{
        TocItem {}
    }
}
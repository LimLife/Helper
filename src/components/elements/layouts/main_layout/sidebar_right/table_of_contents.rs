mod toc_item;
mod header_toc;

use dioxus::prelude::*;
use super::table_of_contents::{
    toc_item::TocItem,
    header_toc::HeaderToc
};

#[component]
pub fn TableOfContents()->Element{
    rsx!{
        div { class: "table-of-contents",
            HeaderToc {}
            nav { class: "table-of-contents-nav",
                TocItem { content: "Function " }
                TocItem { content: "Class" }
                TocItem { content: "Property" }
                TocItem { content: "Object" }
                TocItem { content: "Method" }
                TocItem { content: "Event" }
            }
        }
    }
}
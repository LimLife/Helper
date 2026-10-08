mod header_toc;
mod toc_item;
use super::table_of_contents::{header_toc::HeaderToc, toc_item::TocItem};
use dioxus::prelude::*;
#[component]
pub fn TableOfContents() -> Element {
    rsx! {
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

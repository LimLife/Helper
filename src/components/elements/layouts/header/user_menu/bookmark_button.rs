use dioxus::prelude::*;

use crate::components::assets::images::BOOKMARK;

#[component]
pub fn BookmarkButton()->Element {
    rsx!{
        button { id: "bookmark-button", class: "bookmark-button",
            img { src: BOOKMARK, alt: "Bookmark" }
        }
    }
}
use crate::components::assets::images::BOOKMARK;
use dioxus::prelude::*;
#[component]
pub fn BookmarkButton() -> Element {
    rsx! {
        button { id: "bookmark-button", class: "bookmark-button",
            img { src: BOOKMARK, alt: "Bookmark" }
        }
    }
}

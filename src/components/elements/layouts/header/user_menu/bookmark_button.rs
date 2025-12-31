use dioxus::prelude::*;


#[component]
pub fn BookmarkButton()->Element {
    rsx!{
        span { id: "bookmark-button", class: "bookmark-button", "BookmarkButton" }
    }
}
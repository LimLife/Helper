mod comment;

use dioxus::prelude::*;

use super::comment_preview::comment::Comment;

#[component]
pub fn CommentPreview() -> Element {
    rsx! {
        ul {
            Comment { comment: "First" }
            Comment { comment: "Second" }
            Comment { comment: "Third" }
        }
    }
}
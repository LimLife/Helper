mod comment;
use super::comment_preview::comment::Comment;
use dioxus::prelude::*;
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

use dioxus::prelude::*;


#[component]
pub fn CommentForm() -> Element {
    rsx! {
        textarea { class: "discussion-preview-comment-form", "Comment Form" }
    }
}

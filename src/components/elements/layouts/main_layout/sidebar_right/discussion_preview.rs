mod comment_form;
mod comment_preview;
mod view_all_comments_link;
use super::discussion_preview::{
    comment_form::CommentForm, comment_preview::CommentPreview,
    view_all_comments_link::ViewAllCommentsLink,
};
use dioxus::prelude::*;
#[component]
pub fn DiscussionPreview() -> Element {
    rsx! {
        div { class: "discussion-preview",
            CommentForm {}
            CommentPreview {}
            ViewAllCommentsLink {}
        }
    }
}

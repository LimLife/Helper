mod comment_form;
mod comment_preview;
mod view_all_comments_link;

use dioxus::prelude::*;
use super::discussion_preview::{
    comment_form::CommentForm,
    comment_preview::CommentPreview,
    view_all_comments_link::ViewAllCommentsLink
};

#[component]
pub fn DiscussionPreview() -> Element {
    rsx!{
        div { class: "discussion-preview",
            CommentForm {}
            CommentPreview {}
            ViewAllCommentsLink {}
        }
    }
}

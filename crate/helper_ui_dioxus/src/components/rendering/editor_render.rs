use dioxus::prelude::*;
use domain::{
    data::{Content, InsertPosition},
    id::ContentID,
};

use crate::features::editor::block::article_block_editor::ArticleBlockEditor;

#[component]
pub fn EditorRenderComponent(
    on_create: EventHandler<InsertPosition>,
    on_edit: EventHandler<ContentID>,
    node_block: Vec<Content>,
) -> Element {
    let on_create = move |position| on_create.call(position);
    rsx! {
        for content in node_block {
            ArticleBlockEditor {
                content: content,
                on_create,
                on_edit
            }
        }
    }
}

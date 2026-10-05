use crate::components::elements::editor_elements::article_body_editor::ArticleBodyEditor;

use dioxus::prelude::*;
use shared::manifesto::{Content, InsertPosition};
use shared::new_type_id::ContentID;

#[component]
pub fn RenderComponent(
    on_create: EventHandler<InsertPosition>,
    on_edit: EventHandler<ContentID>,
    node_block: Vec<Content>,
) -> Element {
    let on_create = move |position| on_create.call(position);
    rsx! {
        for content in node_block {
            ArticleBodyEditor {
                content: content,
                on_create,
                on_edit
            }
        }
    }
}

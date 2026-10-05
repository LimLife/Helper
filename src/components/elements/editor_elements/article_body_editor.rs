use super::tool_bar_block::ToolBarBlock;
use crate::components::elements::editor_elements::body_editable::{
    code::CodeEditor, info_box_content::InfoBoxEditor, section_content::SectionTextEditor,
    warning_box_content::WarningBoxEditor,
};
use crate::components::elements::shared_elements::{
    code_block::CodeBlock, info_box::InfoBox, section::SectionText, warning_box::WarningBox,
};

use dioxus::prelude::*;
use shared::manifesto::{Block, Content, InsertPosition};
use shared::new_type_id::ContentID;
use shared::store::EditorStore;
use shared::traits::editor_store::exsist_draft::ExsistDraft;
use shared::traits::editor_store::select::Select;

#[component]
pub fn ContentEditor(
    content: Content,
    on_create: EventHandler<InsertPosition>,
    on_edit: EventHandler<ContentID>,
) -> Element {
    let (content_id, order) = (content.id, content.order);
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let selected = editor_store.read().is_select(content_id);
    let toggle = move |_| {
        editor_store.write().toggle(content_id);
    };
    let block = {
        let editor = editor_store.read();
        if let Some(draft) = editor.draft(content_id) {
            draft.block.clone()
        } else {
            content.block.clone()
        }
    };

    rsx! {
        div { id: "article_body_editor", class: "article_body_editor", ondoubleclick: toggle,
            div { id: "preview", Preview {
                block: block.clone()
            } }
            if selected {
                ToolBarBlock { id: content_id, on_create, on_edit}
                div {
                    id: "editor-editable",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                    },
                    EditBlock {
                        content_id,
                        block,
                        order
                    }
                }
            }
        }
    }
}

#[component]
fn Preview(block: Block) -> Element {
    let preview = match block {
        Block::Code(data) => rsx! {
            CodeBlock {
                code: data.clone()
            }
        },

        Block::Warning(data) => rsx! {
            WarningBox {
                warnig_data: data.clone()
            }
        },

        Block::Section(data) => rsx! {
            SectionText {
                section_data: data.clone()
            }
        },

        Block::Info(data) => rsx! {
            InfoBox {
                info_data: data.clone()
            }
        },
    };
    preview
}

#[component]
fn EditBlock(block: Block, content_id: ContentID, order: u32) -> Element {
    let editor_content = match block {
        Block::Code(data) => {
            rsx! {
                CodeEditor { code_data: data.clone(), id: content_id ,order }
            }
        }
        Block::Warning(data) => {
            rsx! {
                WarningBoxEditor { warnig_data: data.clone(), id: content_id }
            }
        }
        Block::Section(data) => {
            rsx! {
                SectionTextEditor { section_data: data.clone(), id: content_id }
            }
        }
        Block::Info(data) => {
            rsx! {
                InfoBoxEditor { info_data: data.clone(), id: content_id }
            }
        }
    };
    editor_content
}

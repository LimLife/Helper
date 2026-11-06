use application::store::{EditorStore, ExsistDraft, Select};
use dioxus::prelude::*;
use domain::data::{Block, Content, InsertPosition};
use domain::id::ContentID;

use crate::features::articel::block::artilce_section::SectionText;
use crate::features::articel::block::code_block::CodeBlock;
use crate::features::articel::block::info_box::InfoBox;
use crate::features::articel::block::warning_box::WarningBox;
use crate::features::editor::block::editor_block::{
    CodeEditor, InfoEditor, SectionEditor, WarningEditor,
};
use crate::features::editor::ui::block_toolbar::BlockToolbar;

#[component]
pub fn ArticleBlockEditor(
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
                BlockToolbar { id: content_id, on_create, on_edit}
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
                warning_data: data.clone()
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
                WarningEditor { warning_data: data.clone(), id: content_id }
            }
        }
        Block::Section(data) => {
            rsx! {
                SectionEditor { section_data: data.clone(), id: content_id }
            }
        }
        Block::Info(data) => {
            rsx! {
                InfoEditor { info_data: data.clone(), id: content_id }
            }
        }
    };
    editor_content
}

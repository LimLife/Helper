use super::tool_bar_block::ToolBarBlock;
use crate::components::elements::editor_elements::editable::{
    code_editor::CodeEditor, info_box_editor::InfoBoxEditor, section_editor::SectionTextEditor,
    warning_box_editor::WarningBoxEditor,
};
use dioxus::prelude::*;
use shared::manifesto::Block;
use shared::new_type_id::ContentID;
use shared::store::ArticleStore;
use shared::store::EditorStore;

#[component]
pub fn EditorFrame(content_id: ContentID, preview: Element) -> Element {
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let selected = editor_store.read().is_select(content_id.clone());
    let click = move |_| {
        editor_store.write().toggel(content_id.clone());
    };
    let store_block = use_context::<Signal<ArticleStore>>();

    let blcok = {
        let editor = editor_store.read();
        if let Some(draft) = editor.draft(content_id.clone()) {
            Some(draft.clone())
        } else {
            store_block
                .read()
                .current_block(content_id.clone())
                .cloned()
        }
    };
    let editor_content = match blcok {
        Some(Block::Code(data)) => rsx! {
            CodeEditor { code_data: data.clone(), id:content_id.clone()  }
        },
        Some(Block::Warning(data)) => rsx! {
            WarningBoxEditor { warnig_data: data.clone() ,id:content_id.clone()}
        },
        Some(Block::Section(data)) => rsx! {
            SectionTextEditor {  section_data: data.clone(), id:content_id.clone() }
        },
        Some(Block::Info(data)) => rsx! {
            InfoBoxEditor { info_data: data.clone() , id:content_id.clone()}
        },
        None => rsx! {
            div { class: "editor-empty-state", "Блок не найден" }
        },
    };
    rsx! {
        div {
            id: "editor-frame",
            class:"editor-frame",
            onclick: click,
                div {
                    id: "preview",
                    {preview}
                }
            if selected {
                ToolBarBlock{}
                div {
                     id: "editor-editable",
                     onclick: move |e: Event<MouseData>| {
                         e.stop_propagation();
                     },
                     {editor_content}
                }
            }
        }
    }
}

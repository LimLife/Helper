use crate::components::elements::{
    editor_elements::panel_header::PanelHeader, ui::code_view::TextEditor,
};
use crate::data_component::content_saver::Saver;
use dioxus::prelude::*;
use shared::store::ArticleStore;
use shared::{
    data_block::CodeData, manifesto::Content, new_type_id::ContentID, store::EditorStore,
    traits::editor_store::exsist_draft::ExsistDraft,
};

#[component]
pub fn CodeEditor(code_data: CodeData, id: ContentID, order: u32) -> Element {
    type Code = CodeData;
    let original_data = code_data.clone();
    let editor_store = use_context::<Signal<EditorStore>>();
    let article_store = use_context::<Signal<ArticleStore>>();
    let saver = use_context::<Signal<Saver>>();
    let mut edit_data = use_signal(|| code_data.clone());
    if editor_store.read().has_draft(id) {
        if let Some(code) = editor_store.read().draft_as::<Code>(id) {
            edit_data.set(code);
        }
    }
    let save_content = {
        let mut editor_store = editor_store;
        move |_| {
            let Some(draft) = editor_store.with(|data| data.draft(id).cloned()) else {
                return;
            };
            let article_id = draft.article_id;
            let content = Content {
                id: draft.content_id,
                block: draft.block,
                order,
            };
            spawn(async move {
                let result = saver.read().update_content(article_id, content).await;
                match result {
                    Ok(content) => {
                        editor_store.write().discard_draft(content.id);
                    }
                    Err(e) => {
                        debug!("Error saving {e}");
                    }
                }
            });
        }
    };
    let discard_data = {
        let mut editor_store = editor_store;
        let mut c_data = edit_data;
        move |_| {
            editor_store.write().discard_draft(id);
            c_data.set(original_data.clone());
        }
    };

    let on_input = {
        let mut editor_store = editor_store;

        move |e| {
            edit_data.with_mut(|code_content| {
                code_content.code_content = Some(e);
            });

            if editor_store.read().has_draft(id) {
                editor_store
                    .write()
                    .update_draft(id, edit_data.read().clone());
            } else {
                let draft = article_store.read().draft(id).unwrap();

                editor_store.write().set_draft::<CodeData>(
                    draft.order,
                    draft.article_id,
                    draft.content_id,
                    edit_data.read().clone(),
                );
            }
        }
    };
    rsx! {
        div { id: "code-editor", class: "code-editor",
            PanelHeader { save: save_content, remove: discard_data }
            TextEditor {
                value: edit_data.with(|code_contnet| code_contnet.code_content.clone().unwrap_or_default()),
                on_input
            }
        }
    }
}

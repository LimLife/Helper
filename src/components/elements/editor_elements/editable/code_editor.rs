use crate::components::elements::editor_elements::panel_header::PanelHeader;
use dioxus::prelude::*;
use shared::data_block::CodeData;
use shared::new_type_id::ContentID;
use shared::store::EditorStore;
#[component]
pub fn CodeEditor(code_data: CodeData, id: ContentID) -> Element {
    let origina_data = code_data.clone();
    let editor_store = use_context::<Signal<EditorStore>>();
    if editor_store.read().has_draft(id) {
        if let Some(code) = editor_store.read().druft_as::<CodeData>(id) {
            code_data = code;
        }
    }
    let mut c_data = use_signal(|| code_data.clone());
    let click = {
        let mut editor_store = editor_store;
        move |_| {
            editor_store.write().set_draft(id, c_data.read().clone());
        }
    };
    let discard_data = {
        let mut editor_store = editor_store;
        let mut c_data = c_data;
        move |_| {
            editor_store.write().discard_draft(id);
            c_data.set(origina_data.clone());
        }
    };
    use_drop({
        let mut editor_store = editor_store;
        move || {
            editor_store.write().set_draft(id, c_data.read().clone());
        }
    });
    rsx! {
            div {
                id: "code-editor",
                class: "code-editor",
                PanelHeader {
                  save: click,
                  remove: discard_data
                },
                div {
                    class: "code-textarea-pos",
                    textarea {
                        id: "code-code-content",
                        class: "code-textarea",
                        value: c_data.read().code_content.clone(),
                        oninput: move |e| {
                            c_data.with_mut(|data|{
                                data.code_content = Some(e.value());
                            });
                        },
                        placeholder: "Введите или вставьте код...",
                        spellcheck: "false",
                        autocapitalize: "off",
                        autocomplete: "off",
                    }
                }
            }
    }
}

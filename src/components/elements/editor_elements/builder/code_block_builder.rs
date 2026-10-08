use crate::components::elements::{
    editor_elements::panel_header::PanelHeader, ui::code_view::TextEditor,
};
use dioxus::prelude::*;
use shared::{
    data_block::CodeData, editor::BlockEditorContext, store::EditorStore,
    traits::editor_store::exsist_draft::ExsistDraft,
};
#[component]
pub fn CodeBlockBuilder(context: BlockEditorContext, data: CodeData) -> Element {
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let mut init = use_signal(|| false);
    let code = use_signal(|| data);

    use_effect(move || {
        let data = code.read().clone();

        if !init() {
            init.set(true);
            return;
        }

        editor_store.write().create_or_update(context.clone(), data);
    });

    let on_save = { move |_| {} };

    let on_remove = move |_| {};

    rsx! {
        div {
            class: "code-editor",

            CodeFieldEditor {
                data: code,
            }

            CodeContentEditor {
                data: code,
            }

            PanelHeader {
                save: on_save,
                remove: on_remove,
            }
        }
    }
}

#[component]
fn CodeFieldEditor(data: Signal<CodeData>) -> Element {
    rsx! {
        section {
            div {
                class: "block-editor-section",
                div {
                    class: "block-editor-section-header",
                    TextEditor {
                        value: data.with(|title| title.title.clone().unwrap_or_default()),
                        on_input: move |e| {
                            data.with_mut(|title| {
                                title.title = Some(e)
                            })
                        }
                    }
                }
                div {
                    class: "block-editor-section-body",
                    TextEditor {
                        value: data.with(|header| header.code_header.clone().unwrap_or_default()),
                        on_input: move |e| {
                            data.with_mut(|code_header| {
                                code_header.code_header = Some(e)
                            })
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CodeContentEditor(data: Signal<CodeData>) -> Element {
    rsx! {
        section {
            class: "block-editor-section",
            div {
                class: "block-editor-section-header",
                h3 {
                    "Content"
                }
            }
            div {
                class: "block-editor-section-body",
                TextEditor {
                    value: data.with(|code_content| code_content.code_content.clone().unwrap_or_default()),
                    on_input: move |e| {
                        data.with_mut(|code|{
                            code.code_content = Some(e)
                        })
                    }
                }
            }
        }
    }
}

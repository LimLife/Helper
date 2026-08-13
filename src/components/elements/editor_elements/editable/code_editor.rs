use dioxus::prelude::*;
use shared::data_block::CodeData;
use shared::new_type_id::ContentID;
use shared::store::EditorStore;
#[component]
pub fn CodeEditor(code_data: CodeData, id: ContentID) -> Element {
    let mut code_store = use_context::<Signal<EditorStore>>();
    let mut c_data = use_signal(|| code_data.clone());
    let click = move |_| {
        code_store.write().set_draft(
            id,
            CodeData {
                title: code_data.title.clone(),
                code_content: code_data.code_content.clone(),
                code_header: code_data.code_header.clone(),
            },
        );
    };
    rsx! {
            div {
                id: "code-editor",
                class: "code-editor",
                style: "display: flex; flex-direction: column; gap: 12px; width: 100%;",

                // Шапка редактора
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 0 4px;",
                    span {
                        style: "font-size: 11px; font-weight: 600; color: #858585; text-transform: uppercase; letter-spacing: 0.08em;",
                        "Редактор кода"
                    }
                    button {
                        class: "code-btn-save",
                        onclick: click,
                        "Сохранить"
                    }
                }

                // Textarea
                div {
                    style: "width: 100%; position: relative;",
                    textarea {
                        id: "code-code-content",
                        class: "code-textarea",
                        style: "
                            width: 100%;
                            min-width: 100%;
                            max-width: 100%;
                            box-sizing: border-box;
                            min-height: 120px;
                            padding: 16px;
                            border: 1px solid #3e3e42;
                            border-radius: 8px;
                            background: #1e1e1e;
                            color: #d4d4d4;
                            font-family: 'Fira Code', 'Cascadia Code', 'Consolas', 'Courier New', monospace;
                            font-size: 14px;
                            line-height: 1.6;
                            tab-size: 4;
                            resize: vertical;
                            overflow-x: auto;
                            overflow-y: hidden;
                            white-space: pre;
                            word-wrap: normal;
                            field-sizing: content;
                            outline: none;
                            transition: border-color 0.2s, box-shadow 0.2s;
                        ",
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

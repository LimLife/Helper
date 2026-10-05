use dioxus::prelude::*;
use shared::{
    editor::{BlockEditorTarget, NewDraftContent},
    manifesto::{Block, InsertPosition},
    new_type_id::ArticleID,
    store::EditorStore,
    traits::editor_store::new_draft::NewDraft,
};

use crate::components::elements::editor_elements::block_editor::BlockEditor;

#[component]
pub fn BlockBuilder(
    position: InsertPosition,
    on_create: EventHandler<NewDraftContent>,
    on_cancel: EventHandler<()>,
    article_id: ArticleID,
) -> Element {
    let editor_store = use_context::<Signal<EditorStore>>();

    let has_draft = editor_store.read().has_new_draft();

    let cancel = {
        let mut editor_store = editor_store;
        move |_| {
            editor_store.write().cancel_new_draft();
            on_cancel.call(());
        }
    };
    let select_code = {
        let mut editor_store = editor_store;
        move |_| {
            editor_store
                .write()
                .start_new(position, article_id, Block::empty_code());
        }
    };

    let select_info = {
        let mut editor_store = editor_store;
        move |_| {
            editor_store
                .write()
                .start_new(position, article_id, Block::empty_info());
        }
    };

    let select_section = move |_| {
        let mut editor_store = editor_store;
        editor_store
            .write()
            .start_new(position, article_id, Block::empty_section());
    };

    let select_warning = move |_| {
        let mut editor_store = editor_store;
        editor_store
            .write()
            .start_new(position, article_id, Block::empty_warning());
    };
    let create = move |_| {
        let mut editor_store = editor_store;
        let Some(draft) = editor_store.write().take_new_draft() else {
            return;
        };
        on_create.call(draft)
    };
    rsx! {
        div {
            class: "block-builder-overlay",

            div {
                class: "block-builder",

                if has_draft {
                    div {
                        class: "block-builder-editor",

                        BlockEditor {
                            target: BlockEditorTarget::New(article_id),
                            on_cancel
                        }
                    }

                    div {
                        class: "block-builder-footer",

                        button {
                            r#type: "button",
                            class: "block-builder-cancel",
                            onclick: cancel,
                            "Cancel"
                        }

                        button {
                            r#type: "button",
                            class: "block-builder-create",
                            onclick: create,
                            "Create"
                        }
                    }
                } else {
                    div {
                        class: "block-builder-header",

                        h2 {
                            "Create Block"
                        }

                        button {
                            r#type: "button",
                            class: "block-builder-close",
                            onclick: cancel,
                            "×"
                        }
                    }

                    div {
                        class: "block-builder-body",

                        BlockTypeButton {
                            name: "Code",
                            description: "Code example snippet",
                            icon: "</>",
                            onclick: select_code,
                        }

                        BlockTypeButton {
                            name: "Warning",
                            description: "Warning example snippet",
                            icon: "!",
                            onclick: select_warning,
                        }

                        BlockTypeButton {
                            name: "Info",
                            description: "Information block",
                            icon: "i",
                            onclick: select_info,
                        }

                        BlockTypeButton {
                            name: "Section",
                            description: "Section content",
                            icon: "¶",
                            onclick: select_section,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlockTypeButton(
    name: String,
    description: String,
    icon: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "block-type-button",
            onclick,
            div {
                class: "block-type-button-icon",
                {icon}
            }
            div {
                class: "block-type-button-content",
                div {
                    class: "block-type-button-name",
                    {name}
                }
                div {
                    class: "block-type-button-description",
                    {description}
                }
            }
        }
    }
}

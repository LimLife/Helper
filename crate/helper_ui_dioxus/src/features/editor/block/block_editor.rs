use application::{
    editor_state::{BlockEditorContext, BlockEditorTarget},
    store::{ArticleStore, EditorStore, NewDraft},
};
use dioxus::prelude::*;
use domain::data::Block;

use crate::features::editor::block::builder_block::{
    CodeBuilder, InfoBuilder, SectionBuilder, WarningBuilder,
};

#[component]
pub fn BlockEditor(target: BlockEditorTarget, on_cancel: EventHandler<()>) -> Element {
    let editor_store = use_context::<Signal<EditorStore>>();

    let article_store = use_context::<Signal<ArticleStore>>();

    let resolved = match target {
        BlockEditorTarget::Existing(content_id, article_id) => {
            if let Some(draft) = editor_store.read().drafts.get(&content_id).cloned() {
                let context = BlockEditorContext::Existing {
                    article_id: draft.article_id,
                    content_id: draft.content_id,
                    order: draft.order,
                };

                (context, draft.block)
            } else {
                let Some(content) = article_store.read().current_content(content_id).cloned()
                else {
                    return rsx! {
                        div {
                            "Content not found"
                        }
                    };
                };

                let context = BlockEditorContext::Existing {
                    article_id,
                    content_id: content.id,
                    order: content.order,
                };

                (context, content.block)
            }
        }

        BlockEditorTarget::New(article_id) => {
            let Some(draft) = editor_store.read().new_draft().cloned() else {
                return rsx! {
                    div {
                        "New block not found"
                    }
                };
            };

            let context = BlockEditorContext::New {
                article_id,
                position: draft.position,
            };

            (context, draft.block)
        }
    };

    let (context, block) = resolved;

    let content_edit = match block {
        Block::Code(data) => rsx! {
            CodeBuilder {
                context,
                data,
            }
        },

        Block::Warning(data) => rsx! {
            WarningBuilder {
                context,
                data,
            }
        },

        Block::Section(data) => rsx! {
            SectionBuilder {
                context,
                data,
            }
        },

        Block::Info(data) => rsx! {
            InfoBuilder {
                context,
                data,
            }
        },
    };

    rsx! {
        div {
            class: "block-editor",

            BlockEditorHeader {
                target,
                on_cancel,
            }

            div {
                class: "block-editor-body",
                {content_edit}
            }
        }
    }
}

#[component]
fn BlockEditorHeader(target: BlockEditorTarget, on_cancel: EventHandler<()>) -> Element {
    let title = match target {
        BlockEditorTarget::Existing(_, _) => "Edit block",
        BlockEditorTarget::New(_) => "Create block",
    };
    let close = move |_| on_cancel.call(());
    rsx! {
       div {
           class: "block-editor-header",
           div {
               class: "block-editor-header-info",
               h2 {
                   class: "block-editor-title",
                   "{title}"
               }
            }
            button {
                r#type: "button",
                class: "block-editor-close",
                onclick: close,
                "x"
            }
        }
    }
}

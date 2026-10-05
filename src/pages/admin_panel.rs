use crate::{
    components::elements::{
        editor_elements::{block_builder::BlockBuilder, block_editor::BlockEditor},
        layouts::main_layout::sidebar_right::SidebarRight,
        shared_elements::tree_root::TreeRoot,
    },
    render_components::editor_render::RenderComponent,
};

use dioxus::prelude::*;

use shared::{
    editor::{BlockEditorMode, BlockEditorTarget, NewDraftContent},
    manifesto::{InsertPosition, NodeMeta},
    new_type_id::ContentID,
    store::{ArticleStore, EditorStore},
};

#[component]
pub fn AdminPage() -> Element {
    let article_store = use_context::<Signal<ArticleStore>>();
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let mut editor_mode = use_signal(|| None::<BlockEditorMode>);
    let mut blocks = {
        let store = article_store.read();
        store
            .current_body()
            .map(|body| body.blocks.clone())
            .unwrap_or_default()
    };
    blocks.sort_by_key(|contnet| contnet.order);

    let node_right = use_signal(|| vec![NodeMeta::new()]);

    let mut open_create = move |position_type: InsertPosition| {
        editor_mode.set(Some(BlockEditorMode::Create(position_type)));
    };
    let open_editor = move |content_id: ContentID| {
        editor_mode.set(Some(BlockEditorMode::Edit(content_id)));
    };
    let close = move |_| {
        editor_mode.set(None);
    };

    let render_mode = match editor_mode.read().clone() {
        None => VNode::empty(),
        Some(BlockEditorMode::Create(position_type)) => {
            let Some(article_id) = article_store.read().current() else {
                return rsx! {
                    div {
                        "No article selected"
                    }
                };
            };
            rsx! {
                div { id: "block-builder", class: "block-builder-overlay",
                    BlockBuilder {
                        position: position_type,
                        on_create: move | draft: NewDraftContent | {
                           editor_store.write().set_new_draft(draft);
                           editor_mode.set(Some(BlockEditorMode::EditNew));
                        },
                        on_cancel: close,
                        article_id
                    }
                }
            }
        }
        Some(BlockEditorMode::Edit(contnet_id)) => {
            let Some(article_id) = article_store.read().current() else {
                return rsx! {
                    div {
                        "No article selected"
                    }
                };
            };
            rsx! {
                div {
                    id: "block-editor",
                    class: "block-builder-overlay",
                    BlockEditor {
                        target: BlockEditorTarget::Existing(contnet_id, article_id),
                        on_cancel: close
                    }
                }
            }
        }
        Some(BlockEditorMode::EditNew) => {
            let Some(article_id) = article_store.read().current else {
                return rsx! {
                    div {
                        "No article selected"
                    }
                };
            };

            rsx! {
                div {
                    id: "block-editor",
                    class: "block-builder-overlay",

                    BlockEditor {
                        target: BlockEditorTarget::New(article_id),
                        on_cancel: close,
                    }
                }
            }
        }
    };

    let editor_render = if article_store.read().current().is_some() {
        rsx! {
            SelectCreateBlock {
                create: move |_| {open_create(InsertPosition::First)},
            }
            RenderComponent {
                on_create: open_create,
                on_edit: open_editor,
                node_block: blocks,
            }
        }
    } else {
        VNode::empty()
    };
    rsx! {
        div { id: "admin-page", class: "editor-layout",
            div { id: "editor-root-tree", TreeRoot {} }
            div { id: "editor-render",
                { editor_render }
            }
            { render_mode }
            div { id: "editor-right-bar",
                SidebarRight { node_right }
            }
        }
    }
}

#[component]
fn SelectCreateBlock(create: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "admin-page-empty-content",
            onclick: move |_| create.call(()),
            "+"
        }
    }
}

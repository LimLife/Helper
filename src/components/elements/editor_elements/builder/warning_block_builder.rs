use dioxus::prelude::*;
use shared::data_block::WarningData;
use shared::editor::BlockEditorContext;
use shared::store::EditorStore;
use shared::traits::editor_store::exsist_draft::ExsistDraft;

use crate::components::elements::editor_elements::panel_header::PanelHeader;
use crate::components::elements::ui::code_view::TextEditor;

#[component]
pub fn WarningBlockBuilder(context: BlockEditorContext, data: WarningData) -> Element {
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let mut init = use_signal(|| false);
    let mut warning = use_signal(|| data);

    use_effect(move || {
        let data = warning.read().clone();

        if !init() {
            init.set(true);
            return;
        }

        editor_store.write().create_or_update(context.clone(), data);
    });

    let on_save = { move |_| {} };

    let on_remove = move |_| {};

    rsx! {
        TextEditor {
            value: warning.with(|warning| warning.warning.clone().unwrap_or_default()),
            on_input: move |e| {
                warning.with_mut(|warning| warning.warning = Some(e))
            }
        }
        PanelHeader {
            save: on_save,
            remove: on_remove,
        }
    }
}

use application::editor_state::BlockEditorContext;
use application::store::{EditorStore, ExsistDraft};
use dioxus::prelude::*;
use domain::data::WarningData;

use crate::components::code_view::TextEditor;
use crate::features::editor::ui::panel_header::PanelHeader;

#[component]
pub fn WarningBuilder(context: BlockEditorContext, data: WarningData) -> Element {
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

use crate::components::elements::editor_elements::panel_header::PanelHeader;
use crate::components::elements::ui::code_view::TextEditor;
use application::editor_state::BlockEditorContext;
use application::store::{EditorStore, ExsistDraft};
use dioxus::prelude::*;
use domain::data::InfoData;

#[component]
pub fn InfoBlockBuilder(context: BlockEditorContext, data: InfoData) -> Element {
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let mut init = use_signal(|| false);
    let mut info = use_signal(|| data);

    use_effect(move || {
        let data = info.read().clone();

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
            value: info.with(|info| info.info.clone().unwrap_or_default()),
            on_input: move |e| {
                info.with_mut(|info| info.info = Some(e))
            }
        }
            PanelHeader {
                save: on_save,
                remove: on_remove,
            }
    }
}

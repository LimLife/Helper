use dioxus::prelude::*;
use shared::data_block::SectionData;
use shared::editor::BlockEditorContext;
use shared::store::EditorStore;
use shared::traits::editor_store::exsist_draft::ExsistDraft;

use crate::components::elements::editor_elements::panel_header::PanelHeader;
use crate::components::elements::ui::code_view::TextEditor;

#[component]
pub fn SectionBlockBuilder(context: BlockEditorContext, data: SectionData) -> Element {
    let mut editor_store = use_context::<Signal<EditorStore>>();
    let mut init = use_signal(|| false);
    let mut section = use_signal(|| data);

    use_effect(move || {
        let data = section.read().clone();

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
            value: section.with(|section| section.paragraph.clone().unwrap_or_default()),
            on_input: move |e| {
                section.with_mut(|section| section.paragraph = Some(e))
            }
        }
            PanelHeader {
                save: on_save,
                remove: on_remove,
            }
    }
}

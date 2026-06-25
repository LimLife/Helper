use crate::components::elements::layouts::main_layout::sidebar_left::tree::Tree;
use dioxus::prelude::*;
use shared::store::SectionStore;
#[component]
pub fn TreeRoot() -> Element {
    let store = use_context::<Signal<SectionStore>>();
    rsx! {
        for node in store.read().root.iter() {
            Tree {
                node_id: node.clone(),
                deep: 0
            }
        }
    }
}

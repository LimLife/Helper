use crate::components::elements::layouts::main_layout::sidebar_left::tree::Tree;
use application::store::SectionStore;
use dioxus::prelude::*;

#[component]
pub fn TreeRoot() -> Element {
    let store = use_context::<Signal<SectionStore>>();
    rsx! {
        for node in store.read().root.iter() {
            Tree { node_id: node.clone(), deep: 0 }
        }
    }
}

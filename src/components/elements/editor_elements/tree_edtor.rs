use dioxus::prelude::*;

use crate::components::elements::shared_elements::tree_root::TreeRoot;

#[component]
pub fn TreeEditor() -> Element {
    rsx! {
        div {
            id: "tree-editor",
            class: "tree-editor",
        TreeRoot{}
        }
    }
}

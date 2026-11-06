use dioxus::prelude::*;

use crate::features::navigation::tree_root::TreeRoot;
#[component]
pub fn TreeEditor() -> Element {
    rsx! {
        div { id: "tree-editor", class: "tree-editor", TreeRoot {} }
    }
}

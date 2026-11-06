use application::store::SectionStore;
use dioxus::prelude::*;

use crate::{features::navigation::tree::Tree, types::AppLoader};

#[component]
pub fn TreeRoot() -> Element {
    let mut store = use_context::<Signal<SectionStore>>();

    let loader = use_context::<AppLoader>();

    debug!("loaded element tree root");
    use_resource(move || async move {
        if !store.peek().is_root() {
            debug!("store is loaded");
            return;
        }
        match loader.load_root().await {
            Ok(root) => {
                store.write().init_root(root);
            }
            Err(err) => debug!("{:?}", err.to_string()),
        }
    });
    rsx! {
        for node in store.read().root.iter() {
            Tree { node_id: node.clone(), deep: 0 }
        }
    }
}

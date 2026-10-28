use crate::{
    components::elements::layouts::main_layout::sidebar_left::tree::Tree, data_use_cases::AppLoader,
};
use application::store::SectionStore;
use dioxus::prelude::*;

#[component]
pub fn TreeRoot() -> Element {
    let mut store = use_context::<Signal<SectionStore>>();
    let loader = use_context::<Signal<AppLoader>>();

    use_resource(move || async move {
        if !store.peek().is_root() {
            debug!("store is loaded");
            return;
        }
        match loader.peek().load_root().await {
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

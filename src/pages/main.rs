use dioxus::prelude::*;
use shared::store::SectionStore;

use crate::{
    components::elements::layouts::{footer::Footer, header::Header, main_layout::MainLayout},
    data_component::cotent_loader::Loader,
};

#[component]
pub fn Main() -> Element {
    use_effect(move || {
        spawn(async move {
            let loader = use_context::<Loader>();
            let mut store = use_context::<Signal<SectionStore>>();
            if store.read().is_root() {
                let manifest = loader.load_root().await.unwrap();
                store.write().init_root(manifest);
            }
        });
    });
    rsx! {
        div { id: "main-page", class: "main-page",
            Header {}
            MainLayout {}
            Footer {}
        }
    }
}

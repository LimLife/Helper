use dioxus::prelude::*;

use crate::components::elements::layouts::{
    footer::Footer, header::Header, main_layout::MainLayout,
};
use shared::manifesto::RootManifest;

#[component]
pub fn Main() -> Element {
    rsx! {
        div { id: "main-page", class: "main-page",
            Header {}
            MainLayout {}
            Footer {}
        }
    }
}

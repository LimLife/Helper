use crate::components::elements::layouts::{
    footer::Footer, header::Header, main_layout::MainLayout,
};

use dioxus::prelude::*;
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

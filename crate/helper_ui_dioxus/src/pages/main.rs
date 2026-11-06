use dioxus::prelude::*;

use crate::layouts::{
    footer_layout::FooterLayout, header_layout::HeaderLayout, main_layout::MainLayout,
};

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div {
            id: "main-page",
            class: "main-page",
            HeaderLayout{},
            MainLayout {},
            FooterLayout {}
        }
    }
}

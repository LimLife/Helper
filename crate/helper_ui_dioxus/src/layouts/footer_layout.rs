use dioxus::prelude::*;

use crate::layouts::footer::footer_components::{
    FooterBrand, FooterCommunity, FooterCopyright, FooterLanguages, FooterResources,
};
#[component]
pub fn FooterLayout() -> Element {
    rsx! {
        footer { id: "footer", class: "footer",
            FooterBrand {}
            FooterCommunity {}
            FooterCopyright {}
            FooterLanguages {}
            FooterResources {}
        }
    }
}

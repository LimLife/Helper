use dioxus::prelude::*;

use crate::components::assets::images::FAVICON;

#[component]
pub fn Brand() -> Element {
    rsx! {
        div { id: "brand", class: "brand",
            img {
                id: "Logo",
                src: FAVICON,
                alt: "logo",
                width: 100,
                height: 50,
            }
            div { id: "brand-description", class: "brand-description", "Helper" }
        }
    }
}
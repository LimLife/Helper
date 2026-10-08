use crate::components::assets::images::APP;
use dioxus::prelude::*;
#[component]
pub fn Brand() -> Element {
    rsx! {
        div { id: "brand", class: "brand",
            img {
                id: "Logo",
                src: APP,
                alt: "logo",
                width: 100,
                height: 50,
            }
            div {
                id: "brand-description",
                class: "brand-description font-size-titel",
                "Helper"
            }
        }
    }
}

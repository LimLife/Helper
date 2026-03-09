use dioxus::prelude::*;

use crate::components::assets::images::COPY;

#[component]
pub fn CopyButton() -> Element {
    rsx! {
        button { class: "code-block-nav-button",
            img {
                src: COPY,
                width: "20px",
                height: "20px",
                alt: "Copy button",
            }
        }

    }
}
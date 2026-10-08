use crate::components::assets::images::COPY;
use dioxus::prelude::*;
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

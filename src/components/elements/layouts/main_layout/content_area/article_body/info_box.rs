use dioxus::prelude::*;


#[component]
pub fn InfoBox() -> Element {
    rsx!{
        div { class: "artical-body-info-box", "info box" }
    }
}
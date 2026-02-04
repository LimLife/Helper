use dioxus::prelude::*;


#[component]
pub fn InfoBox() -> Element {
    rsx!{
        aside { class: "artical-body-info-box", "info box" }
    }
}
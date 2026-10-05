use dioxus::prelude::*;


#[component]
pub fn WarningBox() -> Element{
    rsx!{
        div { class: "artical-body-warning-box", "warning box" }
    }
}
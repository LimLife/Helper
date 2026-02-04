use dioxus::prelude::*;


#[component]
pub fn WarningBox() -> Element{
    rsx!{
        aside { class: "artical-body-warning-box", "warning box" }
    }
}
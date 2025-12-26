use dioxus::prelude::*;


#[component]
pub fn LeftBar()->Element {
    rsx! {
        div {
            h4 { "Left Bar" }
        }
    }
}
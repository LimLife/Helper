use dioxus::prelude::*;

#[component]
pub fn ExpandArrow() -> Element {
    rsx!{
        div { id: "expand-arrow", class: "expand-arrow", ">" }
    }
}
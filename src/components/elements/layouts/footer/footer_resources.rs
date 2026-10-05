use dioxus::prelude::*;


#[component]
pub fn FooterResources() -> Element {
    rsx!{
        span { class: "footer-resources", "Footer Resources" }
    }
}
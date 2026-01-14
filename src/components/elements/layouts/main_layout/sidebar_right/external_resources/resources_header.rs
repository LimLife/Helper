use dioxus::prelude::*;


#[component]
pub fn ResourcesHeader() -> Element {
    rsx!{
        div { class: "external-resources-header", "External Resources" }
    }
}
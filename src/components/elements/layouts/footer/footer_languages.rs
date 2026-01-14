use dioxus::prelude::*;


#[component]
pub fn FooterLanguages() -> Element {
    rsx!{
        span { class: "footer-languages", "Footer Languages" }
    }
}
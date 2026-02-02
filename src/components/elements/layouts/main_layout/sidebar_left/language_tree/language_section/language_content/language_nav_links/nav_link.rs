use dioxus::prelude::*;


#[component]
pub fn NavLink() -> Element {
    rsx!{
        a { class: "lenguage-section-content-language-nav-link", href: "#", "Nav Link" }
    }
}
use dioxus::prelude::*;


#[component]
pub fn NavLinkItem(href:String, link_name: String) -> Element {
    rsx!{
        a { class: "lenguage-section-content-language-nav-link", href, {link_name} }
    }
}
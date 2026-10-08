mod nav_link;
mod nested_nav_group;
use super::language_nav_links::nested_nav_group::NestedNavGroup;
use dioxus::prelude::*;
#[component]
pub fn LanguageNavLinks() -> Element {
    rsx! {
        div { class: "lenguage-section-content-language-nav", NestedNavGroup {} }
    }
}

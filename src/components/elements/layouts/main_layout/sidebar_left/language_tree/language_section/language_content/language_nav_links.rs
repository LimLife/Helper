mod nav_link;
mod nested_nav_group;

use dioxus::prelude::*;
use super::language_nav_links::{
    nested_nav_group::NestedNavGroup
};

#[component]
pub fn LanguageNavLinks()-> Element{
    // ul ul ul ->li
    rsx!{
        div { class: "lenguage-section-content-language-nav", NestedNavGroup {} }
    }
}
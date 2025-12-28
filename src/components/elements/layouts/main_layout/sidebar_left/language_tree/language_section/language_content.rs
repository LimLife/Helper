mod language_nav_links;
mod version_selector;

use dioxus::prelude::*;


use super::language_content::{
    language_nav_links::LanguageNavLinks,
    version_selector::VersionSelector
};

#[component]
pub fn LanguageContent() -> Element {
    rsx!{
        div { 
            VersionSelector {}
            LanguageNavLinks {},
        }
    }
}

mod language_section;

use dioxus::prelude::*;
use super::language_tree::language_section::LanguageSection;
#[component]
pub fn LanguageTree() -> Element {
    rsx!{
        LanguageSection {}
    }
}
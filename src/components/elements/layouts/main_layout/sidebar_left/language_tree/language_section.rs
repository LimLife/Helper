mod language_content;
mod language_header;

use dioxus::prelude::*;

use super::language_section::{
    language_header::LanguageHeader,
    language_content::LanguageContent
};

#[component]
pub fn LanguageSection()-> Element {
    rsx!{
        details { id: "lenguage-section", class: "lenguage-section",
            summary { LanguageHeader {} }
            LanguageContent {}
        }
    }
}
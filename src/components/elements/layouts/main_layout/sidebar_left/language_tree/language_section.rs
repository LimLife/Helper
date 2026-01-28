mod language_content;
mod language_header;

use dioxus::prelude::*;

use super::language_section::{
    language_header::LanguageHeader,
    language_content::LanguageContent
};
use crate::components::elements::ui::collapse::Collapse;
#[component]
pub fn LanguageSection()-> Element {
    rsx!{
        Collapse {
            id: "language-section",
            label: rsx! {
                LanguageHeader {}
            },
            content: rsx! {
                LanguageContent {}
            },
        }
    }
}
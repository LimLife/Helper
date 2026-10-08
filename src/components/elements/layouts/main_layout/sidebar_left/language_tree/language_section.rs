mod language_content;
mod language_header;
use super::language_section::{language_content::LanguageContent, language_header::LanguageHeader};
use crate::components::elements::ui::collapse::Collapse;
use dioxus::prelude::*;
#[component]
pub fn LanguageSection() -> Element {
    rsx! {
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

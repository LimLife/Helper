mod expand_arrow;
mod language_icon;
mod language_title;
use super::language_header::{language_icon::LanguageIcon, language_title::LanguageTitle};
use dioxus::prelude::*;
#[component]
pub fn LanguageHeader() -> Element {
    rsx! {
        div { id: "language-header", class: "language-header",
            LanguageIcon {}
            LanguageTitle {}
        }
    }
}

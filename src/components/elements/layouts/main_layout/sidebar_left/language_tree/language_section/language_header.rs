mod expand_arrow;
mod language_icon;
mod language_title;

use dioxus::prelude::*;
use super::language_header::{
   // expand_arrow::ExpandArrow,
    language_icon::LanguageIcon,
    language_title::LanguageTitle
};

#[component]
pub fn LanguageHeader() -> Element {
    rsx!{
        div { id: "language-header", class: "language-header",
            LanguageIcon {}
            LanguageTitle {}
                // ExpandArrow {}
        }
    }
}
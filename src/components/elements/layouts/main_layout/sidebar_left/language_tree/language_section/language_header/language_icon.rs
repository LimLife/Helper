use dioxus::prelude::*;

use crate::components::assets::images::PROGRAMMING_LANGUAGE;
#[component]
pub fn LanguageIcon() -> Element {
    rsx!{
        img {
            id: "program-icon",
            class: "language-icon",
            src: PROGRAMMING_LANGUAGE,
            alt: "Language Icon",
            width: "40px",
            height: "40px",
        }
    }
}
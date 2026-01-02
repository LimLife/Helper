use dioxus::prelude::*;


#[component]
pub fn LanguageTitle() -> Element {
    rsx!{
        div { id: "language-title", class: "anguage-title", "Rust" }
    }
}
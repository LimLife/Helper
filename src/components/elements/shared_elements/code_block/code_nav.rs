use dioxus::prelude::*;
mod copy_button;
mod language_badge;
use super::code_nav::{
    copy_button::CopyButton,
    language_badge::LanguageBadge
    };

#[component]
pub fn CodeNav() -> Element {
    rsx! {
        div { class: "code-block-content-nav",
            LanguageBadge {}
            CopyButton {}
        }
    }
}
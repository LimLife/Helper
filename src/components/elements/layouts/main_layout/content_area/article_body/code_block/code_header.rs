mod language_badge;
mod copy_button;


use dioxus::prelude::*;
use super::code_header::{
    language_badge::LanguageBadge,
    copy_button::CopyButton
};

#[component]
pub fn CodeHeader() -> Element {
    rsx! {
        header { class: "code-block-header",
            LanguageBadge {}
            CopyButton {}
        }
    }
}
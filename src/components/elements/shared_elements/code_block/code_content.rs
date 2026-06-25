use dioxus::prelude::*;

use super::code_nav::CodeNav;

#[component]
pub fn CodeContent(content: Option<String>) -> Element {
    rsx! {
        pre { class: "code-block-content-pre",
            CodeNav {}
            code { class: "code-block-content-code", {content.unwrap_or_default()} }
        }
    }
}

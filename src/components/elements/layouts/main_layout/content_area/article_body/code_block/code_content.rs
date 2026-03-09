use dioxus::prelude::*;

use super::code_nav::CodeNav;

#[component]
pub fn CodeContent() -> Element {
    rsx! {
        pre { class: "code-block-content-pre",
            CodeNav {}
            code { class: "code-block-content-code", "use dioxus::prelude::*;" }
        }
    }
}
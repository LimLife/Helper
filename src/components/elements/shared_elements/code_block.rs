mod code_content;
mod code_header;
mod code_nav;

use super::code_block::{code_content::CodeContent, code_header::CodeHeader};
use dioxus::prelude::*;
use shared::data_block::CodeData;

#[component]
pub fn CodeBlock(code: CodeData) -> Element {
    rsx! {
        section { id: "code-block", class: "code-block",
            CodeHeader {}
            CodeContent { content: code.code_content.clone()}
        }
    }
}

use crate::components::elements::shared_elements::code_block::CodeBlock;
use dioxus::prelude::*;
use shared::data_block::CodeData;

#[component]
pub fn CodeEditor(code: CodeData) -> Element {
    rsx! {
        CodeBlock { code }
    }
}

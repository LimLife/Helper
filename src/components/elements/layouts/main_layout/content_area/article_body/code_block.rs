mod code_content;
mod code_header;
mod code_nav;

use dioxus::prelude::*;
use super::code_block::{
    code_header::CodeHeader,
    code_content::CodeContent
};

#[component]
pub fn CodeBlock()->Element{
    rsx!{
        section { id: "code-block", class: "code-block",
            CodeHeader {}
            CodeContent {}
        }
    }
}
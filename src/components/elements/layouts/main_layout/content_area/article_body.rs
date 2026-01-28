mod section;
mod code_block;
mod info_box;
mod warning_box;
mod grid;

use dioxus::prelude::*;
use super::article_body::{
    section::Section,
    code_block::CodeBlock,
    grid::Grid
};

#[component]
pub fn ArticleBody() -> Element {
    rsx!{
        article { id: "artical-main-content", class: "artical-body",
            Section {}
            CodeBlock {}
            Grid {}
        }
    }
}
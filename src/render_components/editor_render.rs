use crate::components::elements::editor_elements::{
    code_editor::CodeEditor, info_box_editor::InfoBoxEditor, section_editor::SectionTextEditor,
    warning_box_editor::WarningBoxEditor,
};

use dioxus::prelude::*;
use shared::manifesto::{Block, Content};

#[component]
pub fn RenderCoomponent(node_block: Vec<Content>) -> Element {
    rsx! {
            for node in node_block.iter() {
                match &node.block {
                    Block::Code(code_data)=> rsx! {
                        CodeEditor {
                            code:code_data.clone()
                        }
                    },
                    Block::Info(info_data_c) => rsx! {
                        InfoBoxEditor {
                            info_block: info_data_c.clone()
                        }
                    },
                    Block::Section(section_data)=> rsx!{
                        SectionTextEditor { section_data:section_data.clone()}
                    },
                    Block::Warning(warnig_data) => rsx! {
                        WarningBoxEditor{ warnig_data:warnig_data.clone()}
                    }
                }
        }
    }
}

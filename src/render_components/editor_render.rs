use crate::components::elements::editor_elements::editor_frame::EditorFrame;
use crate::components::elements::shared_elements::{
    code_block::CodeBlock, info_box::InfoBox, section::SectionText, warning_box::WarningBox,
};
use dioxus::prelude::*;
use shared::manifesto::{Block, Content};

#[component]
pub fn RenderCoomponent(node_block: Vec<Content>) -> Element {
    rsx! {
            for node in node_block.iter() {
                match &node.block {
                Block::Code(code_data) => rsx! {
                    EditorFrame {
                        preview: rsx! {
                            CodeBlock {
                                code: code_data.clone()
                            }
                        },
                        content_id: node.id.clone(),
                    }
                },
                Block::Info(info_data) => rsx! {
                        EditorFrame {
                            preview: rsx!{
                                InfoBox { info_data: info_data.clone()}
                            },
                            content_id: node.id.clone()
                        }
                    },
                Block::Section(section_data)=> rsx! {
                        EditorFrame {
                            preview: rsx!{
                                SectionText { section_data:section_data.clone()}
                            },
                            content_id: node.id.clone()
                        },
                },
                Block::Warning(warnig_data) => rsx! {
                        EditorFrame {
                            preview: rsx!{
                                WarningBox{ warnig_data:warnig_data.clone()}
                            },
                            content_id: node.id.clone()
                        },
                    }
                }
            }
    }
}

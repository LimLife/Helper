use crate::features::articel::block::{
    artilce_section::SectionText, code_block::CodeBlock, info_box::InfoBox, warning_box::WarningBox,
};
use dioxus::prelude::*;
use domain::data::{Block, Content};
#[component]
pub fn RenderComponent(node_block: Vec<Content>) -> Element {
    rsx! {
        for node in node_block.iter() {
            match &node.block {
                Block::Code(code_data) => rsx! {
                    CodeBlock { code: code_data.clone() }
                },
                Block::Info(info_data_c) => rsx! {
                    InfoBox { info_data: info_data_c.clone() }
                },
                Block::Section(section_data) => rsx! {
                    SectionText { section_data: section_data.clone() }
                },
                Block::Warning(warnig_data) => rsx! {
                    WarningBox { warning_data: warnig_data.clone() }
                },
            }
        }
    }
}

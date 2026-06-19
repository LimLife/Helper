use crate::components::elements::shared_elements::{
    code_block::CodeBlock, info_box::InfoBox, section::SectionText, warning_box::WarningBox,
};
use dioxus::prelude::*;
use shared::manifesto::Block;

#[component]
pub fn RenderCoomponent(node_block: Signal<Vec<Block>>) -> Element {
    let bloks = node_block.read();
    rsx! {
            for node in bloks.iter() {
                match node {
                    Block::Code(code_data)=> rsx! {
                        CodeBlock { code:code_data.clone() }
                    },
                    Block::Info(info_data_c) => rsx! {
                        InfoBox { info_block: info_data_c.clone() }
                    },
                    Block::Section(section_data)=> rsx!{
                        SectionText {section_data:section_data.clone()}
                    },
                    Block::Warning(warnig_data) => rsx! {
                        WarningBox{warnig_data:warnig_data.clone()}
                    }
                }
        }
    }
}

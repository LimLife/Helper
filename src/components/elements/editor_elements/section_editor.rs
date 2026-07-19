use crate::components::elements::shared_elements::section::SectionText;
use dioxus::prelude::*;
use shared::data_block::SectionData;
#[component]
pub fn SectionTextEditor(section_data: SectionData) -> Element {
    rsx! {
        div {
            id: "section-editor",
            class: "section-editor",
        SectionText{section_data}
        }
    }
}

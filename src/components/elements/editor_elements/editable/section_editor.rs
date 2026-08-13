use crate::components::elements::shared_elements::section::SectionText;
use dioxus::prelude::*;
use shared::data_block::SectionData;
use shared::new_type_id::ContentID;

#[component]
pub fn SectionTextEditor(section_data: SectionData, id: ContentID) -> Element {
    rsx! {
        div {
            id: "section-editor",
            class: "section-editor",
            SectionText{section_data}
        }
    }
}

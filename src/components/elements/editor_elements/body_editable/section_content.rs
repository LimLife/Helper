use crate::components::elements::shared_elements::section::SectionText;
use dioxus::prelude::*;
use domain::{data::SectionData, id::ContentID};

#[component]
pub fn SectionTextEditor(section_data: SectionData, id: ContentID) -> Element {
    rsx! {
        div { id: "section-editor", class: "section-editor",
            SectionText { section_data }
        }
    }
}

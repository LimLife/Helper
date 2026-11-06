use dioxus::prelude::*;
use domain::{data::SectionData, id::ContentID};

use crate::features::articel::block::artilce_section::SectionText;

#[component]
pub fn SectionEditor(section_data: SectionData, id: ContentID) -> Element {
    rsx! {
        div { id: "section-editor", class: "section-editor",
            SectionText { section_data }
        }
    }
}

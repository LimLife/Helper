mod paragraph;
mod section_title;

use super::section::{paragraph::Paragraph, section_title::SectionTitle};
use dioxus::prelude::*;
use shared::data_block::SectionData;

#[component]
pub fn SectionText(section_data: SectionData) -> Element {
    rsx! {
        SectionTitle { title: section_data.title },
        Paragraph { parafraph: section_data.paragraph },
    }
}

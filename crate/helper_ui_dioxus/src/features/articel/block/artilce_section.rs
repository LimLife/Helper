pub mod paragraph;
pub mod section_title;
use dioxus::prelude::*;
use domain::data::SectionData;

use crate::features::articel::block::artilce_section::{
    paragraph::Paragraph, section_title::SectionTitle,
};

#[component]
pub fn SectionText(section_data: SectionData) -> Element {
    rsx! {
        SectionTitle { title: section_data.title }
        Paragraph { parafraph: section_data.paragraph }
    }
}

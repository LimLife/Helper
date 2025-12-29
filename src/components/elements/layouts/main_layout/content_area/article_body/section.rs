mod paragraph;
mod section_title;


use dioxus::prelude::*;
use super::section::{
    paragraph::Paragraph,
    section_title::SectionTitle
};

#[component]
pub fn Section()->Element{
    rsx!{
        div {  
            Paragraph {  },
            SectionTitle {},
        }
    }
}
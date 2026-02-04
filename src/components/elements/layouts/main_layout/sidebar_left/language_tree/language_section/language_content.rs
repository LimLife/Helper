mod language_nav_links;


use dioxus::prelude::*;


use super::language_content::{
    language_nav_links::LanguageNavLinks,
};
use crate::components::models::selector_version::selector::Selector;
use crate::components::elements::ui::version_selector::VersionSelector;
#[component]
pub fn LanguageContent() -> Element {
    let select = vec![
        Selector { disabled:false, name: String::from("First"), selected:false, value: String::from("#")},
        Selector { disabled:false, name: String::from("Second"), selected:false, value: String::from("#")},
        Selector { disabled:false, name: String::from("Third"), selected:true, value: String::from("#")},
        Selector { disabled:false, name: String::from("Fourth"), selected:false, value: String::from("#")}
    ];
    rsx!{
        div { class: "lenguage-section-content",
            VersionSelector { selector: select.clone() }
            LanguageNavLinks {}
        }
    }
}

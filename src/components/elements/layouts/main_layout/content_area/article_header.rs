mod article_description;
mod article_title;



use dioxus::prelude::*;
use super::article_header::{
    article_description::ArticleDescription,
    article_title::ArticleTitle
};
use crate::components::elements::ui::version_selector::VersionSelector;
use crate::components::models::selector_version::selector::Selector;
#[component]
pub fn ArticleHeader() -> Element {
     let select = vec![
        Selector { disabled:false, name: String::from("First"), selected:false, value: String::from("#")},
        Selector { disabled:false, name: String::from("Second"), selected:false, value: String::from("#")},
        Selector { disabled:false, name: String::from("Third"), selected:true, value: String::from("#")},
        Selector { disabled:false, name: String::from("Fourth"), selected:false, value: String::from("#")}
    ];
    rsx!{
        header { id: "artical-header", class: "artical-header",
            div { class: "artical-header-text",
                ArticleTitle {}
                ArticleDescription {}
            }
            VersionSelector { selector: select.clone() }
        }
    }
}
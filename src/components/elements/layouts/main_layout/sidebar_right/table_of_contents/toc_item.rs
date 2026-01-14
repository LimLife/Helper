use dioxus::prelude::*;


#[component]
pub fn TocItem(content: String)->Element{
    rsx!{
        div { {content} }
    }
}
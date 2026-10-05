use dioxus::prelude::*;


#[component]
pub fn ExternalLink(link:String) -> Element {
    rsx!{
        li { {link} }
    }
}
use dioxus::prelude::*;

#[component]
pub fn PrevArticleLink(prev: Option<String>, prev_name: Option<String>) -> Element {
    rsx! {
        a { href: prev.unwrap_or_default(), {prev_name.unwrap_or_default()} }
    }
}

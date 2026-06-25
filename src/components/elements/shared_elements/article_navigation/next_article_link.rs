use dioxus::prelude::*;

#[component]
pub fn NextArticleLink(next: Option<String>, next_name: Option<String>) -> Element {
    rsx! {
        a { href: next.unwrap_or_default(), {next_name.unwrap_or_default()} }
    }
}

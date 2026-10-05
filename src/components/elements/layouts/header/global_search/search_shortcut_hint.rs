use dioxus::prelude::*;


#[component]
pub fn SearchShortcutHint() -> Element {
    rsx! {
        span { id: "search-shortcut", class: "search-shortcut", "SearchShortcutHint" }
    }
}
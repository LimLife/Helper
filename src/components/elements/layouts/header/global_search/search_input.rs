use dioxus::prelude::*;

#[component]
pub fn SearchInput()->Element {
    rsx!{

        input {
            id: "search-input",
            class: "search-input",
            placeholder: "SerchInput",
            r#type: "text",
            aria_label: "Search",
        }
        span { id: "hint", class: "search-hint", "⌘K" }

    }
}
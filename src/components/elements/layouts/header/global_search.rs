mod search_input;
mod search_results_dropdown;
mod search_shortcut_hint;

use dioxus::prelude::*;

use super::global_search::{
    search_input::SearchInput,
    search_results_dropdown::SearchResultDropDown,
    search_shortcut_hint::SearchShortcutHint
};

#[component]
pub fn GlobalSearch() -> Element {
    rsx!{
        div { id: "global-search", class: "global-search",
            SearchInput {}
            SearchResultDropDown {}
            SearchShortcutHint {}
        }
    }
}

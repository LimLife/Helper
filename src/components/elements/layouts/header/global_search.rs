mod search_input;
mod search_results_dropdown;
mod search_shortcut_hint;

use dioxus::prelude::*;

use super::global_search::{
    search_input::SearchInput,
    search_results_dropdown::SearchResultDropDown,
    search_shortcut_hint::SearchShortcutHint
};

pub fn GlobalSearch() -> Element {
    rsx!{
        div { 
            SearchInput{},
            SearchResultDropDown{},
            SearchShortcutHint{}
        }
    }
}

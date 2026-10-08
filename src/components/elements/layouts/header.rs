mod brand;
mod global_search;
mod user_menu;
use super::header::{brand::Brand, global_search::GlobalSearch, user_menu::UserMenu};
use dioxus::prelude::*;
#[component]
pub fn Header() -> Element {
    rsx! {
        header { id: "header", class: "header",
            Brand {}
            GlobalSearch {}
            UserMenu {}
        }
    }
}

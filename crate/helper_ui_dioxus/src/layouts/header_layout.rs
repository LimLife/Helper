use dioxus::prelude::*;

use crate::features::{global_search::GlobalSearch, user_menu::UserMenu};
use crate::layouts::header::brand::Brand;

#[component]
pub fn HeaderLayout() -> Element {
    rsx! {
        header {
            id: "header",
            class: "header",
                Brand {},
                GlobalSearch {},
                UserMenu {}
        }
    }
}

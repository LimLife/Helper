mod brand;
mod global_search;
mod user_menu;

use dioxus::prelude::*;

use super::header::{
    brand::Brand,
    global_search::GlobalSearch,
    user_menu::UserMenu,
};

#[component]
pub fn Header()->Element {
    rsx!{
        header { 
            id:"header",
            class:"header",
            Brand{},
            GlobalSearch {},
            UserMenu {}
        }
    }
}

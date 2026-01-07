use dioxus::prelude::*;

mod main;
mod ui;
mod layout;
mod page;
mod component;

use super::css::{
    layout::{
        HEADER,
        MAIN_LAYOUT,
        FOOTER,
        LEFTSIDEBAR
    },
};

pub fn import_layout_styles()->Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_LAYOUT }
        document::Link { rel: "stylesheet", href: HEADER }
        document::Link { rel: "stylesheet", href: FOOTER }
        document::Link { rel: "stylesheet", href: LEFTSIDEBAR }
    }
}
use dioxus::prelude::*;

pub const MAIN_CSS: Asset = asset!("/assets/styles/main.css");



pub fn import_styles()->Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
    }
}
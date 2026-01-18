use dioxus::prelude::*;

use crate::components::assets::images::HISTORY;
#[component]
pub fn HistoryButton() -> Element {
    rsx! {
        button { id: "history-button", class: "history-button",
            img { alt: "hestory", src: HISTORY }
        }
    }
}
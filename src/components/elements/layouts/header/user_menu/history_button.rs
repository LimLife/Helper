use crate::components::assets::images::HISTORY;
use dioxus::prelude::*;
#[component]
pub fn HistoryButton() -> Element {
    rsx! {
        button { id: "history-button", class: "history-button",
            img { alt: "hestory", src: HISTORY }
        }
    }
}

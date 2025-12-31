use dioxus::prelude::*;


#[component]
pub fn HistoryButton() -> Element {
    rsx! {
        span { id: "history-button", class: "history-button", "HistoryButton" }
    }
}
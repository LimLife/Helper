use dioxus::prelude::*;


#[component]
pub fn LanguageBadge() -> Element {
    rsx! {
        span { class: "code-block-nav-badge", "Language Badge" }
    }
}

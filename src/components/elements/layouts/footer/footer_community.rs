use dioxus::prelude::*;


#[component]
pub fn FooterCommunity() -> Element {
    rsx!{
        span { class: "footer-community", "Footer Community" }
    }
}
use dioxus::prelude::*;

#[component]
pub fn FooterCopyright() -> Element {
    rsx! {
        span { class: "footer-copyright", "Footer Copyright" }
    }
}
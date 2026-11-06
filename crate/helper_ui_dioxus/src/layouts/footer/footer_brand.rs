use dioxus::prelude::*;
#[component]
pub fn FooterBrand() -> Element {
    rsx! {
        span { class: "footer-brand", "footer brand" }
    }
}

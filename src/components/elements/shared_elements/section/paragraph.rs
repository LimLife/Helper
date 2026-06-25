use dioxus::prelude::*;

#[component]
pub fn Paragraph(parafraph: Option<String>) -> Element {
    rsx! {
        h2 { {parafraph.unwrap_or_default()} }
    }
}

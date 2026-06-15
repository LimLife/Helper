use dioxus::prelude::*;

#[component]
pub fn SectionTitle(title: Option<String>) -> Element {
    rsx! {
        p {
            {title.unwrap_or_default()}
        }
    }
}

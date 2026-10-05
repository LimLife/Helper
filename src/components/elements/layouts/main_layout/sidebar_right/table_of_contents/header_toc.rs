use dioxus::prelude::*;

#[component]
pub fn HeaderToc()->Element{
    rsx! {
        h3 { class: "table-of-contents-header", "On This Page" }
    }
}
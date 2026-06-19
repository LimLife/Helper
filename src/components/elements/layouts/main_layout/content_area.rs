use dioxus::prelude::*;

#[component]
pub fn ContentArea(children: Element) -> Element {
    rsx! {
        div { id: "content-area", class: "content-area",
           {children}
        }
    }
}

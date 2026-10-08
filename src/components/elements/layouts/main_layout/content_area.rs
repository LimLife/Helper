use dioxus::prelude::*;

use shared::manifesto::NodeMeta;
#[component]
pub fn ContentArea(node_contnet: Signal<Vec<NodeMeta>>) -> Element {
    rsx! {
        div { id: "content-area", class: "content-area",

        }
    }
}

use dioxus::prelude::*;

use crate::components::elements::ui::collapse::Collapse;
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {  }
        Collapse {
            id: "Collaspe-1",
            label: "collapse-1",
            content: rsx!{
                div {
                    "Qwerty-1"
                }
            }
        },
                Collapse {
            id: "Collaspe-2",
            label: "collapse-2",
            content: rsx!{
                div {
                    "Qwerty-2"
                }
            }
        },
                Collapse {
            id: "Collaspe-3",
            label: "collapse-3",
            content: rsx!{
                div {
                    "Qwerty-3"
                }
            }
        },
        div {
            style: "display:grid; justify-content:center;",


            label {
                id: "search",
                "Search"
            }
            input {
                r#type: "search",
            placeholder: "Type here to search...",
            form: "#search"
        }
    }
    }
}

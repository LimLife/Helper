pub mod collapsibale;

use dioxus::prelude::*;

#[component]
pub fn Collapse(id: String, content:Element)->Element {
    rsx!{
        div { class: "collapse",
            input {
                r#type: "checkbox",
                id: "{id}"
            }

            label {
                class: "collapse-label",
                r#for: "{id}",
                "Открыть / Закрыть"
            }

            div { class: "collapse-content-wrapper",
                div {
                    class: "collapse-content",
                       {content}
                    
                }
            }
        }
    }
}
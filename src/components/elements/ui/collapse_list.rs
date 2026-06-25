use dioxus::prelude::*;

#[component]
pub fn CollapseList(id: String, label: Element, content: Element, expand: bool) -> Element {
    rsx! {
        li {
            class: "tree-node",
            button {
                class: "tree-node-trigger",
                r#type: "button",
                aria_controls: "{id}",

                {label}
            }
            if expand {
                ul {
                    id: "{id}",
                    class: "tree-node-children",
                    {content}
                }
            }
        }
    }
}

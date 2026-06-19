use dioxus::prelude::*;

#[component]
pub fn CollapseList(id: String, label: Element, content: Element) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        li {
            class: "tree-node",
            button {
                class: "tree-node-trigger",
                r#type: "button",
                aria_expanded: open(),
                aria_controls: "{id}",
                onclick: move |_| {
                    open.set(!open());
                },
                {label}
            }
            if open() {
                ul {
                    id: "{id}",
                    class: "tree-node-children",
                    {content}
                }
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Collapse(id: String, label: Element, content: Element) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "ui_collapse",
            button {
                class: "ui_collapse-trigger",
                r#type: "button",
                aria_expanded: open(),
                aria_controls: "{id}",
                onclick: move |_| open.set(!open()),
                {label}
            }
            div {
                class: "ui_collapse-content-wrapper",
                class: if open() { "open" } else { "" },
                id: "{id}",
                {content}
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn ToolBarBlock() -> Element {
    rsx! {
        ul {
            id: "tool-bar-block",
            class: "tool-bar-block",
            li {
                id: "move-up",
                "Move UP"
            }
            li {
                id: "move-down",
                "Move DOWN"
            }
            li {
                id: "dublicate",
                "DUBLICATE"
            }
            li {
                id: "delete",
                "DELETE"
            }
            li {
                id: "collapse",
                "COLLAPSE"
            }
            li {
                id: "insert above",
                "INSERT ABOVE"
            }
            li {
                id: "insert below",
                "INSERT BELOW"
            }
        }
    }
}

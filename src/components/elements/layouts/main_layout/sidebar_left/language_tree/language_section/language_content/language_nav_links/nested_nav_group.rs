use dioxus::prelude::*;

use super::nav_link::NavLink;

#[component]
pub fn NestedNavGroup() -> Element{
    rsx!{
        div { class: "lenguage-section-content-language-nav-group",
            span { "Group Test nav link" }
            ul {
                li { NavLink {} }
                li { NavLink {} }
                li { NavLink {} }
            }
        }
    }
}
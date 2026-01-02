use dioxus::prelude::*;



#[component]
pub fn SidebarHeader() -> Element {
    rsx!{
        div { id: "left-sidebar-header", class: "left-sidebar-header", "Helper" }
    }
}
use dioxus::prelude::*;


#[component]
pub fn BreadcrumbItem() -> Element {
    rsx!{
        li { class: "breadcrumbs-item",
            Link { to: "#", "Breadcrumb Item" }
        }
    }
}
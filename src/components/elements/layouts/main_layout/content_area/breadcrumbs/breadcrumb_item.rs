use dioxus::prelude::*;


#[component]
pub fn BreadcrumbItem( link:String, name:String) -> Element {
    rsx!{
        li { class: "breadcrumbs-item",
            Link { to: link, {name} }
        }
    }
}
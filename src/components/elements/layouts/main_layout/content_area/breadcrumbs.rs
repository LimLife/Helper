mod breadcrumb_item;


use dioxus::prelude::*;
use super::breadcrumbs::breadcrumb_item::BreadcrumbItem;

#[component]
pub fn Breadcrumb() -> Element {

    rsx!{
        ul { class: "breadcrumbs",
            BreadcrumbItem {}
            BreadcrumbItem {}
            BreadcrumbItem {}
            BreadcrumbItem {}
        }
    }
}
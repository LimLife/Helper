mod breadcrumb_item;


use dioxus::prelude::*;
use super::breadcrumbs::breadcrumb_item::BreadcrumbItem;
use crate::components::models::content_area::breadcrumb::Breadcrumb;
#[component]
pub fn BreadcrumbView() -> Element {
    let items = vec![
        Breadcrumb { link: String::from("#"), name: String::from("First") },
        Breadcrumb { link: String::from("#"), name: String::from("Second") },
        Breadcrumb { link: String::from("#"), name: String::from("Third") },
        Breadcrumb { link: String::from("#"), name: String::from("Foure") },
    ];
    rsx!{
        ul { class: "breadcrumbs",
            for item in &items {
                BreadcrumbItem { link: item.link.clone(), name: item.name.clone() }
            }
        }
    }
}
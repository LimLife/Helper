use dioxus::prelude::*;
use shared::data_block::BreadcrumbData;

#[component]
pub fn BreadcrumbItem(breadcrumb: Signal<Vec<BreadcrumbData>>) -> Element {
    let data = breadcrumb.read();
    rsx! {
        ul{
            {
                {data.iter().map(|item|{
                        rsx! {
                            li { class: "breadcrumbs-item",
                                Link { to: item.link.clone().unwrap_or_default(), {item.name.clone().unwrap_or_default()} }
                            }
                        }
                })}
            }
        }
    }
}

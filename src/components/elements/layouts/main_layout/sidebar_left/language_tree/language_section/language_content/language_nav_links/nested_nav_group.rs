use dioxus::prelude::*;

use super::nav_link::NavLinkItem;
use crate::components::elements::ui::collapse::Collapse;
use crate::components::models::language_content::{
    nav_group_model::NavGroup,
    nav_link_model::NavLink
};


#[component]
pub fn NestedNavGroup() -> Element {
   let root_group = NavGroup {
        name: "Main Navigation".to_string(),
        links: vec![],
        childrens: Some(vec![
            NavGroup {
                name: "Languages".to_string(),
                links: vec![
                    NavLink { path: "Rust".to_string(), href: "#".to_string() },
                    NavLink { path: "TypeScript".to_string(), href: "#".to_string() },
                ],
                childrens: Some(vec![
                    NavGroup {
                        name: "Frontend".to_string(),
                        links: vec![
                            NavLink { path: "React".to_string(), href: "#".to_string() },
                        ],
                        childrens: None,
                    }
                ]),
            },
            NavGroup {
                name: "Tools".to_string(),
                links: vec![],
                childrens: None,
            },
        ]),
    };
    rsx!{
        div { class: "lenguage-section-content-language-nav-group",
            NavGroupItem { group: root_group }
        }
    }
}

#[component]
pub fn NavGroupItem(group: NavGroup, #[props(default = 0)]level:u32) -> Element {
    let id_name =format!("{}-{}", group.name.clone(), level);
    rsx!{
        Collapse {
            //style: format!("padding-left:{level}rem;"),
            //key: "{group.name.clone()}-{level}",
            id: id_name,
            label: rsx! {
                span { {group.name} }
            },
            content: rsx! {
                if !group.links.is_empty() {
                    ul {
                        for link in &group.links {
                            li {
                                NavLinkItem { href: link.href.clone(), link_name: link.path.clone() }
                            }
                        }
                    }
                }
                if let Some(children) = &group.childrens {
                    for child in children {
                        NavGroupItem { group: child.clone(), level: level + 1 }
                    }
                }
            },
        }
    }
}

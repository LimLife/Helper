use dioxus::prelude::*;

use crate::components::elements::ui::collapse_list::CollapseList;
use shared::{manifesto::NodeType, new_type_id::NodeMetaID, store::SectionStore};
#[component]
pub fn Tree(node_id: NodeMetaID, deep: u32) -> Element {
    let store = use_context::<Signal<SectionStore>>();
    let (node, expand, children) = {
        let store_read = store.read();
        let Some(node) = store_read.nodes.get(&node_id) else {
            return rsx! {
                div { "Loading..." }
            };
        };
        (
            node.clone(),
            store_read.is_expanded(&node_id),
            store_read
                .children
                .get(&node_id)
                .cloned()
                .unwrap_or_default(),
        )
    };

    let padding = format!("padding-left: {}rem;", deep as f64 * 1.25);

    match &node.type_node {
        NodeType::Section => {
            rsx! {
                CollapseList {
                    id: node.id.to_string(),
                    label: rsx! {
                        div {
                            onclick:
                                move |_| {
                                let mut store = use_context::<Signal<SectionStore>>();
                                if !store.read().loaded.contains(&node_id){

                                }
                                store.write().toggle_expand(node_id);
                            },
                            style: "{padding}",
                            "{node.title}"
                        }
                    },
                    content: rsx! {
                        if expand {
                            for child_id in children {
                                Tree {
                                    node_id: child_id,
                                    deep: deep + 1,
                                }
                            }
                        }
                    }
                }
            }
        }

        NodeType::Article { artical_id: _ } => {
            rsx! {
                li {
                    style: "{padding}",

                    button {
                        class: "tree-article",
                        onclick: move |_| {
                            //click to open article
                        },
                        "{node.title}"
                    }
                }
            }
        }
    }
}

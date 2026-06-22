use dioxus::prelude::*;

use crate::components::elements::ui::collapse_list::CollapseList;
use shared::{
    manifesto::NodeType,
    new_type_id::NodeMetaID,
    store::{ArticleStore, SectionStore},
};
#[component]
pub fn Tree(node_id: NodeMetaID, deep: u32) -> Element {
    let store_section = use_context::<Signal<SectionStore>>();
    let mut store_article = use_context::<Signal<ArticleStore>>();
    let (node, expand, children) = {
        let store_read = store_section.read();
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
                    expand: expand.clone(),
                    label: rsx! {
                        div {
                            onclick:
                                move |_| {
                                let mut store = use_context::<Signal<SectionStore>>();
                                if !store.read().loaded.contains(&node_id){
                                    //loaded node
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

        NodeType::Article { article_id } => {
            let article = *article_id;
            rsx! {
                li {
                    style: "{padding}",

                    div {
                        class: "tree-article",
                        onclick: move |_| {
                            store_article.write().open(article);
                        },
                        "{node.title}"
                    }
                }
            }
        }
    }
}

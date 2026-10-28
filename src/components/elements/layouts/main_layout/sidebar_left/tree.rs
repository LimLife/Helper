use crate::{components::elements::ui::collapse_list::CollapseList, data_use_cases::AppLoader};
use application::store::{ArticleStore, SectionStore};
use dioxus::prelude::*;
use domain::{data::NodeType, id::NodeMetaID};

#[component]
pub fn Tree(node_id: NodeMetaID, deep: u32) -> Element {
    let store_section = use_context::<Signal<SectionStore>>();
    let store_article = use_context::<Signal<ArticleStore>>();
    let loader = use_context::<Signal<AppLoader>>();
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
    match node.type_node {
        NodeType::Section => {
            rsx! {
                CollapseList {
                    id: node.id.to_string(),
                    expand: expand.clone(),
                    label: rsx! {
                        div {
                            onclick: move |_| {
                                let mut store = store_section;
                                let loader = loader;
                                async move {
                                    if !store.read().is_loaded(node.id) {
                                        let children = loader.read().load_children(node.id).await.unwrap();
                                        store.write().insert_children(node.id.clone(), children);
                                    }
                                    store.write().toggle_expand(node_id);
                                }
                            },
                            style: "{padding}",
                            "{node.title}"
                        }
                    },
                    content: rsx! {
                        if expand {
                            for child_id in children {
                                Tree { node_id: child_id, deep: deep + 1 }
                            }
                        }
                    },
                }
            }
        }
        NodeType::Article { article_id } => {
            rsx! {
                li { style: "{padding}",
                    div {
                        class: "tree-article",
                        onclick: move |_| {
                            let mut store_article = store_article;
                            let loader = loader;
                            async move {
                                if !store_article.read().has_article(article_id) {
                                    let article = loader.read().article(article_id).await;

                                    if let Ok(article) = article {
                                        store_article.write().insert_article(article);
                                    }
                                }

                                if !store_article.read().has_body(article_id) {
                                    let body = loader.read().load_article_body(article_id).await;

                                    if let Ok(body) = body {
                                        store_article.write().insert_body(body);
                                    }
                                }

                                store_article.write().open(article_id);
                            }
                        },
                        "{node.title}"
                    }
                }
            }
        }
    }
}

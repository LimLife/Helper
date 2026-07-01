use dioxus::prelude::*;

use crate::components::elements::layouts::main_layout::sidebar_left::tree::Tree;

use shared::manifesto::{Article, NodeMeta, NodeType};
use shared::new_type_id::NodeMetaID;
use shared::store::{ArticleStore, SectionStore};

#[component]
pub fn AdminPage() -> Element {
    let mut sections = use_context::<Signal<SectionStore>>();
    let mut articles = use_context::<Signal<ArticleStore>>();

    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);

    let selected = {
        let store = sections.read();
        store.selected.and_then(|id| store.nodes.get(&id).cloned())
    };

    if let Some(node) = &selected {
        if title.read().is_empty() {
            title.set(node.title.clone());
        }

        if description.read().is_empty() {
            description.set(node.description.clone().unwrap_or_default());
        }
    }

    rsx! {

        div { class: "admin-page",

            div { class: "admin-tree",

                for root in sections.read().root.clone() {

                    Tree {
                        node_id: root,
                        deep: 0,
                    }

                }

            }

            div { class: "admin-editor",

                h2 { "Node editor" }

                if let Some(node) = selected {

                    p { "Id: {node.id}" }

                    input {
                        value: "{title}",
                        oninput: move |e| title.set(e.value())
                    }

                    textarea {
                        value: "{description}",
                        oninput: move |e| description.set(e.value())
                    }

                    button {

                        onclick: move |_| {

                            let mut store = sections.write();

                            if let Some(node) = store.nodes.get_mut(&node.id) {

                                node.title = title();
                                node.description =
                                    Some(description());

                            }

                        },

                        "Save"

                    }

                    button {

                        onclick: move |_| {

                           // sections.write().remove_node(node.id);

                        },

                        "Delete"

                    }

                    button {

                        onclick: move |_| {

                            let mut store = sections.write();
                               let id_new = NodeMetaID::new();
                               let node = NodeMeta {

                                    id: NodeMetaID::new(),

                                    parent_id: Some(node.id),

                                    sort_order: 0,

                                    title: "New Section".into(),

                                    description: None,

                                    icon: None,

                                    has_children: false,

                                    type_node: NodeType::Section,
                                };
                            store.nodes.insert(id_new, node);

                        },

                        "+ Section"

                    }

                    button {

                        onclick: move |_| {

                            let article = Article::new(
                                "New article".into(),
                                "Admin".into()
                            );

                            let article_id = article.id;

                            articles.write().insert_article(article);
                            /*
                            sections.write().insert_node(
                                NodeMeta {

                                    id: NodeMetaID::new(),

                                    parent_id: Some(node.id),

                                    sort_order: 0,

                                    title: "New article".into(),

                                    description: None,

                                    icon: None,

                                    has_children: false,

                                    type_node: NodeType::Article {
                                        artical_id: article_id,
                                    },
                                }
                            );
                            */
                        },

                        "+ Article"

                    }

                } else {

                    p {
                        "Select node"
                    }

                }

            }

        }

    }
}

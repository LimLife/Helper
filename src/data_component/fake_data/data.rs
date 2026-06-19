use std::collections::{HashMap, HashSet};

use shared::{
    manifesto::{NodeMeta, NodeType},
    new_type_id::{ArticleID, NodeMetaID},
    store::SectionStore,
};

pub fn fake_section_store() -> SectionStore {
    let mut store = SectionStore::default();

    // Root sections
    let rust_id = NodeMetaID::new();
    let dioxus_id = NodeMetaID::new();

    // Nested section
    let memory_id = NodeMetaID::new();

    // Articles
    let ownership_id = NodeMetaID::new();
    let borrow_id = NodeMetaID::new();
    let signals_id = NodeMetaID::new();

    let ownership_article = ArticleID::new();
    let borrow_article = ArticleID::new();
    let signals_article = ArticleID::new();

    // --------------------------
    // ROOTS
    // --------------------------

    store.root = vec![rust_id, dioxus_id];

    // --------------------------
    // NODES
    // --------------------------

    store.nodes.insert(
        rust_id,
        NodeMeta {
            id: rust_id,
            parent_id: None,
            sort_order: 0,
            title: "Rust".into(),
            description: Some("Rust language".into()),
            icon: None,
            has_children: true,
            type_node: NodeType::Section,
        },
    );

    store.nodes.insert(
        dioxus_id,
        NodeMeta {
            id: dioxus_id,
            parent_id: None,
            sort_order: 1,
            title: "Dioxus".into(),
            description: Some("Dioxus framework".into()),
            icon: None,
            has_children: true,
            type_node: NodeType::Section,
        },
    );

    store.nodes.insert(
        memory_id,
        NodeMeta {
            id: memory_id,
            parent_id: Some(rust_id),
            sort_order: 0,
            title: "Memory".into(),
            description: None,
            icon: None,
            has_children: true,
            type_node: NodeType::Section,
        },
    );

    store.nodes.insert(
        ownership_id,
        NodeMeta {
            id: ownership_id,
            parent_id: Some(memory_id),
            sort_order: 0,
            title: "Ownership".into(),
            description: None,
            icon: None,
            has_children: false,
            type_node: NodeType::Article {
                artical_id: ownership_article,
            },
        },
    );

    store.nodes.insert(
        borrow_id,
        NodeMeta {
            id: borrow_id,
            parent_id: Some(memory_id),
            sort_order: 1,
            title: "Borrow Checker".into(),
            description: None,
            icon: None,
            has_children: false,
            type_node: NodeType::Article {
                artical_id: borrow_article,
            },
        },
    );

    store.nodes.insert(
        signals_id,
        NodeMeta {
            id: signals_id,
            parent_id: Some(dioxus_id),
            sort_order: 0,
            title: "Signals".into(),
            description: None,
            icon: None,
            has_children: false,
            type_node: NodeType::Article {
                artical_id: signals_article,
            },
        },
    );

    // --------------------------
    // CHILDREN
    // --------------------------

    store.children.insert(rust_id, vec![memory_id]);

    store
        .children
        .insert(memory_id, vec![ownership_id, borrow_id]);

    store.children.insert(dioxus_id, vec![signals_id]);

    // --------------------------
    // LOADED
    // --------------------------

    store.loaded = HashSet::from([rust_id, dioxus_id, memory_id]);

    // Для тестов можно раскрыть Rust сразу

    store.expand.insert(rust_id);

    store
}

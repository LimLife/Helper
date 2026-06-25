use std::collections::HashSet;

use shared::{
    data_block::{CodeData, InfoData, SectionData, WarningData},
    manifesto::{Article, ArticleBody, Block, Content, NodeMeta, NodeType},
    new_type_id::{ArticleID, ContentID, NodeMetaID},
    store::{ArticleStore, SectionStore},
};

pub fn fake_data() -> (SectionStore, ArticleStore) {
    let mut store_art = ArticleStore::default();

    // --------------------------
    // ARTICLE 1
    // --------------------------

    let article1 = Article::new("Getting Started".into(), "Dioxus Team".into());

    let body1 = ArticleBody {
        article_id: article1.id,
        bloks: vec![
            Content {
                id: ContentID::new(),
                order: 0,
                block: Block::Section(SectionData {
                    title: Some("Introduction".into()),
                    paragraph: Some("Welcome to the fake article.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 1,
                block: Block::Info(InfoData {
                    info: Some("This is an information block.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 2,
                block: Block::Code(CodeData {
                    title: Some("Hello World".into()),
                    code_header: Some("Rust".into()),
                    code_content: Some(
                        r#"fn main() {
    println!("Hello, world!");
}"#
                        .into(),
                    ),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 3,
                block: Block::Warning(WarningData {
                    warning: Some("Do not copy this into production.".into()),
                }),
            },
        ],
    };

    // --------------------------
    // ARTICLE 2
    // --------------------------

    let article2 = Article::new("Signals".into(), "Dioxus Team".into());

    let body2 = ArticleBody {
        article_id: article2.id,
        bloks: vec![
            Content {
                id: ContentID::new(),
                order: 0,
                block: Block::Section(SectionData {
                    title: Some("Signals".into()),
                    paragraph: Some("Signals are reactive state containers.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 1,
                block: Block::Code(CodeData {
                    title: Some("Signal Example".into()),
                    code_header: Some("Rust".into()),
                    code_content: Some(r#"let mut count = use_signal(|| 0);"#.into()),
                }),
            },
        ],
    };

    // --------------------------
    // ARTICLE 3
    // --------------------------

    let article3 = Article::new("Components".into(), "Dioxus Team".into());

    let body3 = ArticleBody {
        article_id: article3.id,
        bloks: vec![Content {
            id: ContentID::new(),
            order: 0,
            block: Block::Section(SectionData {
                title: Some("Components".into()),
                paragraph: Some("Components are reusable UI units.".into()),
            }),
        }],
    };

    // --------------------------
    // STORE
    // --------------------------

    store_art.insert_article(article1.clone());
    store_art.insert_article(article2.clone());
    store_art.insert_article(article3.clone());

    store_art.insert_body(body1);
    store_art.insert_body(body2);
    store_art.insert_body(body3);

    // Открываем первую статью по умолчанию

    // store_art.open(article1.id);

    let mut store_sec = SectionStore::default();

    // Root sections
    let rust_id = NodeMetaID::new();
    let dioxus_id = NodeMetaID::new();

    // Nested section
    let memory_id = NodeMetaID::new();

    // Articles
    let ownership_id = NodeMetaID::new();
    let borrow_id = NodeMetaID::new();
    let signals_id = NodeMetaID::new();

    // --------------------------
    // ROOTS
    // --------------------------

    store_sec.root = vec![rust_id, dioxus_id];

    // --------------------------
    // NODES
    // --------------------------

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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
                article_id: article1.id,
            },
        },
    );

    store_sec.nodes.insert(
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
                article_id: article2.id,
            },
        },
    );

    store_sec.nodes.insert(
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
                article_id: article3.id,
            },
        },
    );

    // --------------------------
    // CHILDREN
    // --------------------------

    store_sec.children.insert(rust_id, vec![memory_id]);

    store_sec
        .children
        .insert(memory_id, vec![ownership_id, borrow_id]);

    store_sec.children.insert(dioxus_id, vec![signals_id]);

    // --------------------------
    // LOADED
    // --------------------------

    store_sec.loaded = HashSet::from([rust_id, dioxus_id, memory_id]);

    // Для тестов можно раскрыть Rust сразу

    //store.expand.insert(rust_id);

    (store_sec, store_art)
}
pub fn fake_section_store() -> SectionStore {
    let mut store_sec = SectionStore::default();

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

    store_sec.root = vec![rust_id, dioxus_id];

    // --------------------------
    // NODES
    // --------------------------

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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

    store_sec.nodes.insert(
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
                article_id: ownership_article,
            },
        },
    );

    store_sec.nodes.insert(
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
                article_id: borrow_article,
            },
        },
    );

    store_sec.nodes.insert(
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
                article_id: signals_article,
            },
        },
    );

    // --------------------------
    // CHILDREN
    // --------------------------

    store_sec.children.insert(rust_id, vec![memory_id]);

    store_sec
        .children
        .insert(memory_id, vec![ownership_id, borrow_id]);

    store_sec.children.insert(dioxus_id, vec![signals_id]);

    // --------------------------
    // LOADED
    // --------------------------

    store_sec.loaded = HashSet::from([rust_id, dioxus_id, memory_id]);

    // Для тестов можно раскрыть Rust сразу

    //store.expand.insert(rust_id);

    store_sec
}

pub fn fake_article_store() -> ArticleStore {
    let mut store = ArticleStore::default();

    // --------------------------
    // ARTICLE 1
    // --------------------------

    let article1 = Article::new("Getting Started".into(), "Dioxus Team".into());

    let body1 = ArticleBody {
        article_id: article1.id,
        bloks: vec![
            Content {
                id: ContentID::new(),
                order: 0,
                block: Block::Section(SectionData {
                    title: Some("Introduction".into()),
                    paragraph: Some("Welcome to the fake article.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 1,
                block: Block::Info(InfoData {
                    info: Some("This is an information block.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 2,
                block: Block::Code(CodeData {
                    title: Some("Hello World".into()),
                    code_header: Some("Rust".into()),
                    code_content: Some(
                        r#"fn main() {
    println!("Hello, world!");
}"#
                        .into(),
                    ),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 3,
                block: Block::Warning(WarningData {
                    warning: Some("Do not copy this into production.".into()),
                }),
            },
        ],
    };

    // --------------------------
    // ARTICLE 2
    // --------------------------

    let article2 = Article::new("Signals".into(), "Dioxus Team".into());

    let body2 = ArticleBody {
        article_id: article2.id,
        bloks: vec![
            Content {
                id: ContentID::new(),
                order: 0,
                block: Block::Section(SectionData {
                    title: Some("Signals".into()),
                    paragraph: Some("Signals are reactive state containers.".into()),
                }),
            },
            Content {
                id: ContentID::new(),
                order: 1,
                block: Block::Code(CodeData {
                    title: Some("Signal Example".into()),
                    code_header: Some("Rust".into()),
                    code_content: Some(r#"let mut count = use_signal(|| 0);"#.into()),
                }),
            },
        ],
    };

    // --------------------------
    // ARTICLE 3
    // --------------------------

    let article3 = Article::new("Components".into(), "Dioxus Team".into());

    let body3 = ArticleBody {
        article_id: article3.id,
        bloks: vec![Content {
            id: ContentID::new(),
            order: 0,
            block: Block::Section(SectionData {
                title: Some("Components".into()),
                paragraph: Some("Components are reusable UI units.".into()),
            }),
        }],
    };

    // --------------------------
    // STORE
    // --------------------------

    store.insert_article(article1.clone());
    store.insert_article(article2.clone());
    store.insert_article(article3.clone());

    store.insert_body(body1);
    store.insert_body(body2);
    store.insert_body(body3);

    // Открываем первую статью по умолчанию

    store.open(article1.id);

    store
}

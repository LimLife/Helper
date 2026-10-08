use crate::components::shared::card_type_date::{CardDate, CardTypeDate};
use shared::{
    manifesto::{Article, ArticleBody, NodeMeta, NodeType},
    new_type_id::NodeMetaID,
};

pub fn nodeVec() -> Vec<NodeMeta> {
    let article_data = vec![
        Article::new(String::from("Aticle-1"), String::from("You-1")),
        Article::new(String::from("Aticle-2"), String::from("You-2")),
        Article::new(String::from("Aticle-3"), String::from("You-3")),
        Article::new(String::from("Aticle-4"), String::from("You-4")),
        Article::new(String::from("Aticle-5"), String::from("You-5")),
        Article::new(String::from("Aticle-6"), String::from("You-6")),
    ];
    let article_body = vec![
        ArticleBody::new(article_data[0].id),
        ArticleBody::new(article_data[1].id),
        ArticleBody::new(article_data[2].id),
        ArticleBody::new(article_data[3].id),
        ArticleBody::new(article_data[4].id),
        ArticleBody::new(article_data[5].id),
    ];
    let node_pull = vec![
        NodeMeta {
            id: NodeMetaID::new(),
            parent_id: None,
            sort_order: 0,
            description: Some(String::from("Tets-1")),
            has_children: true,
            icon: None,
            title: String::from("Tets-1"),
            type_node: NodeType::Section,
        },
        NodeMeta {
            id: NodeMetaID::new(),
            parent_id: None,
            sort_order: 1,
            description: Some(String::from("Tets-2")),
            has_children: true,
            icon: None,
            title: String::from("Tets-2"),
            type_node: NodeType::Section,
        },
        NodeMeta {
            id: NodeMetaID::new(),
            parent_id: None,
            sort_order: 2,
            description: Some(String::from("Tets-3")),
            has_children: true,
            icon: None,
            title: String::from("Tets-3"),
            type_node: NodeType::Article {
                artical_id: article_body[0].article_id,
            },
        },
        NodeMeta {
            id: NodeMetaID::new(),
            parent_id: None,
            sort_order: 3,
            description: Some(String::from("Tets-4")),
            has_children: true,
            icon: None,
            title: String::from("Tets-4"),
            type_node: NodeType::Article {
                artical_id: article_body[1].article_id,
            },
        },
    ];
    node_pull
}

pub fn cards() -> Vec<CardDate> {
    let card_types = vec![CardDate {
        label: "Frontend".to_string(),
        typs: Some(vec![
            CardTypeDate {
                type_name: "React".to_string(),
                type_description: "UI library for building interactive interfaces".to_string(),
                code: None,
            },
            CardTypeDate {
                type_name: "Dioxus".to_string(),
                type_description: "Rust-based UI framework".to_string(),
                code: None,
            },
            CardTypeDate {
                type_name: "Vue".to_string(),
                type_description: "Progressive JavaScript framework".to_string(),
                code: None,
            },
        ]),
    }];
    card_types
}

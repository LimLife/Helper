use serde::{Deserialize, Serialize};

use crate::type_id::{ArticleID, NodeMetaID};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
    #[default]
    Section,
    Article {
        article_id: ArticleID,
    },
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct NodeMeta {
    pub id: NodeMetaID,
    pub parent_id: Option<NodeMetaID>,
    pub sort_order: u32,
    pub title: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub has_children: bool,
    pub type_node: NodeType,
}
impl NodeMeta {
    pub fn new() -> Self {
        Self {
            id: NodeMetaID::new(),
            parent_id: Default::default(),
            sort_order: Default::default(),
            title: Default::default(),
            icon: Default::default(),
            description: Default::default(),
            has_children: Default::default(),
            type_node: Default::default(),
        }
    }
}

use super::data_block::*;
use super::new_type_id::{ArticleID, ContentID, NodeMetaID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootManifest {
    pub version_bar: u32,
    pub site_title: String,
    pub root_sections: Vec<NodeMeta>,
}

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
//temp-remove-future
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Article {
    pub id: ArticleID,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl Article {
    pub fn new(title: String, author: String) -> Self {
        Self {
            id: ArticleID::new(),
            title,
            author,
            created_at: Default::default(),
            updated_at: Default::default(),
            tags: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArticleBody {
    pub article_id: ArticleID,
    pub bloks: Vec<Content>,
}
impl ArticleBody {
    pub fn new(id: ArticleID) -> Self {
        Self {
            article_id: id,
            bloks: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Content {
    pub id: ContentID,
    pub order: u32,
    pub block: Block,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Code(CodeData),
    Warning(WarningData),
    Section(SectionData),
    Info(InfoData),
}

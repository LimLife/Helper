use crate::new_type_id::{ArticleID, ContentID, NodeMetaID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootManifest {
    pub version_bar: u32,
    pub site_title: String,
    pub root_sections: Vec<NodeMeta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
    Section,
    Article { artical_id: ArticleID },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    pub id: ArticleID,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleBody {
    pub article_id: ArticleID,
    pub bloks: Vec<Content>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    pub id: ContentID,
    pub order: u32,
    pub block: Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Code {
        content: String,
        lang: String,
        alt: String,
        sizex: u32,
        sizey: u32,
    },
    //....
}

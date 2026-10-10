use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::type_id::ArticleID;
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

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct NodeMetaID(pub Uuid);
impl NodeMetaID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for NodeMetaID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ArticleID(pub Uuid);
impl ArticleID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ContentID(pub Uuid);
impl ContentID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

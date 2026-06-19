use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CodeData {
    pub title: Option<String>,
    pub code_content: Option<String>,
    pub code_header: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarningData {
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SectionData {
    pub title: Option<String>,
    pub paragraph: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct InfoData {
    pub info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BreadcrumbData {
    pub link: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArticleNavigationData {
    pub prev: Option<String>,
    pub prev_name: Option<String>,
    pub next: Option<String>,
    pub next_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArticleHeaderData {
    pub description: Option<String>,
    pub title: Option<String>,
}

use application::service::NodeRepository;
use domain::{
    data::{NodeMeta, NodeType},
    id::{ArticleID, NodeMetaID},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresNodeRepository {
    pool: PgPool,
}

impl PostgresNodeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl NodeRepository for PostgresNodeRepository {
    async fn root(&self) -> Result<Vec<NodeMeta>, String> {
        let rows = sqlx::query_as!(NodeMetaRow, r#"SELECT id, parent_id, sort_order, title, icon, description, has_children, type_node,article_id
           FROM node_meta where parent_id is null order by sort_order
           "#).fetch_all(&self.pool).await.map_err(|err| err.to_string())?;
        rows.into_iter().map(NodeMeta::try_from).collect()
    }

    async fn children(&self, parent_id: domain::id::NodeMetaID) -> Result<Vec<NodeMeta>, String> {
        let rows = sqlx::query_as!(NodeMetaRow, r#"SELECT id, parent_id, sort_order, title, icon, description, has_children, type_node, article_id
            FROM node_meta where parent_id = $1 order by sort_order
            "#, parent_id.0).fetch_all(&self.pool).await.map_err(|err| err.to_string())?;
        rows.into_iter().map(NodeMeta::try_from).collect()
    }
}

#[derive(Debug, sqlx::FromRow)]
struct NodeMetaRow {
    id: Uuid,
    parent_id: Option<Uuid>,
    sort_order: i32,
    title: String,
    icon: Option<String>,
    description: Option<String>,
    has_children: bool,
    type_node: String,
    article_id: Option<Uuid>,
}

impl TryFrom<NodeMetaRow> for NodeMeta {
    type Error = String;
    fn try_from(value: NodeMetaRow) -> Result<Self, String> {
        let type_node = match value.type_node.as_str() {
            "section" => {
                if value.article_id.is_some() {
                    return Err("section must not have article_id".into());
                }
                NodeType::Section
            }
            "article" => {
                let article_id = value
                    .article_id
                    .ok_or_else(|| String::from("article must not have article_id"))?;
                NodeType::Article {
                    article_id: ArticleID(article_id),
                }
            }
            unknown_type => return Err(format!("unknown type_node: {unknown_type}")),
        };
        Ok(NodeMeta {
            id: NodeMetaID(value.id),
            parent_id: value.parent_id.map(NodeMetaID),
            sort_order: value.sort_order as u32,
            title: value.title,
            icon: value.icon,
            description: value.description,
            has_children: value.has_children,
            type_node,
        })
    }
}

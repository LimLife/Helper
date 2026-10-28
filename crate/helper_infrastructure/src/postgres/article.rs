use application::service::ArticleRepository;
use chrono::{DateTime, Utc};
use domain::{
    data::{Article, ArticleBody, Block, Content},
    id::{ArticleID, ContentID},
};
use sqlx::{PgPool, query_as, query_scalar};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresArticleRepository {
    pool: PgPool,
}

impl PostgresArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ArticleRepository for PostgresArticleRepository {
    async fn article(&self, article_id: ArticleID) -> Result<domain::data::Article, String> {
        let row = query_as!(
            ArticleRow,
            r#"
                SELECT
                    id,
                    title,
                    author,
                    created_at,
                    updated_at,
                    tags
                FROM articles
                WHERE id = $1
                "#,
            article_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Article not found".to_string())?;
        Ok(Article {
            id: ArticleID(row.id),
            title: row.title,
            author: row.author,
            created_at: row.created_at,
            updated_at: row.updated_at,
            tags: row.tags.unwrap_or_default(),
        })
    }

    async fn article_body(
        &self,
        article_id: ArticleID,
    ) -> Result<domain::data::ArticleBody, String> {
        let exist: bool = query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM articles WHERE id = $1)"#,
            article_id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())?
        .unwrap_or(false);
        if !exist {
            return Err("Article not found".to_string());
        }
        let rows = sqlx::query!(
            r#"
                SELECT
                    id,
                    "order",
                    block
                FROM contents
                WHERE article_id = $1
                ORDER BY "order"
                "#,
            article_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let blocks = rows
            .into_iter()
            .map(|row| {
                let block: Block = serde_json::from_value(row.block).map_err(|e| e.to_string())?;

                Ok(Content {
                    id: ContentID(row.id),
                    order: row.order as u32,
                    block,
                })
            })
            .collect::<Result<Vec<Content>, String>>()?;

        Ok(ArticleBody { article_id, blocks })
    }
}
struct ArticleRow {
    id: Uuid,
    title: String,
    author: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    tags: Option<Vec<String>>,
}

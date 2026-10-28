use application::service::ContentRepository;
use domain::{
    data::Content,
    id::{ArticleID, ContentID},
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostgresContentRepository {
    pool: PgPool,
}

impl PostgresContentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ContentRepository for PostgresContentRepository {
    async fn create(&self, article_id: ArticleID, content: Content) -> Result<ContentID, String> {
        let block_json = serde_json::to_value(&content.block).map_err(|e| e.to_string())?;
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO contents (id,article_id, "order", tags, block)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            content.id.0,
            article_id.0,
            content.order as i32,
            &[] as &[String],
            block_json
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(ContentID(id))
    }

    async fn update(&self, article_id: ArticleID, content: Content) -> Result<ContentID, String> {
        let block_json = serde_json::to_value(&content.block).map_err(|e| e.to_string())?;

        let id = sqlx::query_scalar!(
            r#"
                UPDATE contents
                SET
                    "order" = $1,
                    tags    = $2,
                    block   = $3
                WHERE id = $4
                  AND article_id = $5
                RETURNING id
                "#,
            content.order as i32,
            &[] as &[String],
            block_json,
            content.id.0,
            article_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Content not found".to_string())?;

        Ok(ContentID(id))
    }

    async fn delete(&self, article_id: ArticleID, content_id: ContentID) -> Result<(), String> {
        let result = sqlx::query!(
            r#"
                DELETE FROM contents
                WHERE id = $1
                  AND article_id = $2
                "#,
            content_id.0,
            article_id.0
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Content not found".to_string());
        }

        Ok(())
    }
}

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow, Debug, serde::Serialize)]
pub struct Document {
    pub id: i32,
    pub markdown_content: String,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn create_connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn create_document(pool: &PgPool, content: &str) -> Result<Document, sqlx::Error> {
    let doc = sqlx::query_as!(
        Document,
        r#"
        INSERT INTO documents (markdown_content, updated_at)
        VALUES ($1, NOW())
        RETURNING id, markdown_content, updated_at
        "#,
        content
    )
    .fetch_one(pool)
    .await?;

    Ok(doc)
}

pub async fn get_document(pool: &PgPool, doc_id: i32) -> Result<Option<Document>, sqlx::Error> {
    let doc = sqlx::query_as!(
        Document,
        r#"
        SELECT id, markdown_content, updated_at
        FROM documents 
        WHERE id = $1
        "#,
        doc_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(doc)
}
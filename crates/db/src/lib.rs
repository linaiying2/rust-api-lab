<<<<<<< HEAD
use sqlx::{PgPool, Error, FromRow};
use serde::{Serialize, Deserialize}; // 新增serde导入

#[derive(FromRow, Debug, Serialize, Deserialize)] // 派生serde trait
pub struct User {
    pub id: i32,
    pub name: String,
}

// 获取用户列表
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
}

// 创建用户
pub async fn create_user(pool: &PgPool, user: &User) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
        .bind(user.id)
        .bind(&user.name)
        .execute(pool)
        .await?;
    Ok(())
=======
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
>>>>>>> d624ae11aed19a06cc2a02386e38faa5ea737fef
}
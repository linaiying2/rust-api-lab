use sqlx::{PgPool, Error, FromRow, postgres::PgPoolOptions};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// 保留用户模块（从分支1继承，支持序列化/反序列化）
#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

// 保留文档模块（从分支2继承，支持序列化）
#[derive(FromRow, Debug, Serialize)]
pub struct Document {
    pub id: i32,
    pub markdown_content: String,
    pub updated_at: Option<DateTime<Utc>>,
}

// 保留连接池创建函数（从分支2继承，规范连接池配置）
pub async fn create_connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

// 保留用户查询函数（从分支1继承）
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
}

// 保留用户创建函数（从分支1继承）
pub async fn create_user(pool: &PgPool, user: &User) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
        .bind(user.id)
        .bind(&user.name)
        .execute(pool)
        .await?;
    Ok(())
}

// 保留文档创建函数（从分支2继承）
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

// 保留文档查询函数（从分支2继承）
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
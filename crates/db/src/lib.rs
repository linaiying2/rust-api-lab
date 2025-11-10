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
}
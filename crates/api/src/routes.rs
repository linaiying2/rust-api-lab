use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};
use db::Document;
use serde::Deserialize;
use sqlx::PgPool;

/// 请求体：创建文档
#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub content: String,
}

/// 响应体：返回给前端
#[derive(serde::Serialize)]
pub struct DocumentResponse {
    pub id: i32,
    pub markdown_content: String,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 健康检查
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("API is healthy!")
}

/// 创建文档
pub async fn create_document(
    pool: web::Data<PgPool>,
    req: web::Json<CreateDocumentRequest>,
) -> Result<HttpResponse> {
    let doc = sqlx::query_as!(
        Document,
        r#"
        INSERT INTO documents (markdown_content, updated_at)
        VALUES ($1, NOW())
        RETURNING id, markdown_content, updated_at
        "#,
        req.content
    )
    .fetch_one(&**pool)
    .await;

    match doc {
        Ok(d) => {
            let resp = DocumentResponse {
                id: d.id,
                markdown_content: d.markdown_content,
                updated_at: d.updated_at,
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        Err(e) => {
            eprintln!("Failed to create document: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to create document"))
        }
    }
}

/// 获取文档
pub async fn get_document(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let doc_id = path.into_inner();

    let doc = sqlx::query_as!(
        Document,
        r#"
        SELECT id, markdown_content, updated_at
        FROM documents
        WHERE id = $1
        "#,
        doc_id
    )
    .fetch_optional(&**pool)
    .await;

    match doc {
        Ok(Some(d)) => {
            let resp = DocumentResponse {
                id: d.id,
                markdown_content: d.markdown_content,
                updated_at: d.updated_at,
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        Ok(None) => Ok(HttpResponse::NotFound().body("Document not found")),
        Err(e) => {
            eprintln!("Failed to get document: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to get document"))
        }
    }
}

/// 路由配置函数：供 main.rs 调用
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/documents", web::post().to(create_document))
            .route("/documents/{id}", web::get().to(get_document)),
    );
}
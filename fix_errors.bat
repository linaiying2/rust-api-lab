@echo off
echo Fixing type conflicts and unused imports...

echo 1. Fixing routes.rs...
(
use actix_web::{web, HttpResponse, Result};
use db::Document;
use models::document::{CreateDocumentRequest, DocumentResponse};
use sqlx::PgPool;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("API is healthy!")
}

pub async fn create_document(
    pool: web::Data<PgPool>,
    req: web::Json<CreateDocumentRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let doc = sqlx::query_as!(
        Document,
        "INSERT INTO documents (markdown_content, updated_at) VALUES ($1, NOW()) RETURNING id, markdown_content, updated_at",
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

pub async fn get_document(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let doc_id = path.into_inner();
    let doc = sqlx::query_as!(
        Document,
        "SELECT id, markdown_content, updated_at FROM documents WHERE id = $1",
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

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/documents", web::post().to(create_document))
            .route("/documents/{id}", web::get().to(get_document)),
    );
}
) > crates\api\src\routes.rs

echo 2. Fixing document_service.rs...
(
use db::Document;
use sqlx::PgPool;

pub struct DocumentService;

impl DocumentService {
    pub async fn create_document(pool: &PgPool, content: &str) -> Result<Document, String> {
        if content.trim().is_empty() {
            return Err("Document content cannot be empty".to_string());
        }
        
        db::create_document(pool, content)
            .await
            .map_err(|e| format!("Failed to create document: {}", e))
    }
    
    pub async fn get_document_with_validation(pool: &PgPool, doc_id: i32) -> Result<Option<Document>, String> {
        let doc = db::get_document(pool, doc_id)
            .await
            .map_err(|e| format!("Failed to get document: {}", e))?;
            
        Ok(doc)
    }
}
) > crates\api\src\services\document_service.rs

echo 3. Removing unused imports from websocket_service.rs...
(
use actix::prelude::*;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct MarkdownUpdate {
    pub document_id: i32,
    pub content: String,
}

pub struct MarkdownSyncService {
    sessions: Vec<Recipient<MarkdownUpdate>>,
}

impl MarkdownSyncService {
    pub fn new() -> Self {
        Self { sessions: Vec::new() }
    }
    
    pub fn broadcast_update(&self, update: MarkdownUpdate) {
        for session in &self.sessions {
            let _ = session.do_send(update.clone());
        }
    }
}

impl Actor for MarkdownSyncService {
    type Context = Context<Self>;
}
) > crates\api\src\services\websocket_service.rs

echo Fix completed!
pause
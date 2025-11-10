use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub markdown_content: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocumentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct DocumentResponse {
    pub id: i32,
    pub markdown_content: String,
    pub updated_at: Option<DateTime<Utc>>,
}
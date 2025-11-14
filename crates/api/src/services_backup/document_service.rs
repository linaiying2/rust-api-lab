use db::Document;
use models::document::Document as ModelDocument;  // 避免命名冲突
use sqlx::PgPool;

pub struct DocumentService;

impl DocumentService {
    pub async fn create_document(pool: &PgPool, content: &str) -> Result<ModelDocument, String> {
        // 业务逻辑：内容验证、处理等
        if content.trim().is_empty() {
            return Err("文档内容不能为空".to_string());
        }
        
        // 调用数据层
        db::create_document(pool, content)
            .await
            .map_err(|e| format!("创建文档失败: {}", e))
    }
    
    pub async fn get_document_with_validation(pool: &PgPool, doc_id: i32) -> Result<Option<ModelDocument>, String> {
        let doc = db::get_document(pool, doc_id)
            .await
            .map_err(|e| format!("获取文档失败: {}", e))?;
            
        // 业务逻辑：文档访问权限检查等
        Ok(doc)
    }
}
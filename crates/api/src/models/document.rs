// 定义文档结构体（根据业务需求补充字段）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub id: i32,
    pub content: String,
    // 可添加其他字段（如创建时间、版本号等）
}
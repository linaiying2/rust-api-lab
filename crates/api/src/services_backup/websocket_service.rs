use actix::prelude::*;
use serde_json::json;

pub fn dummy_function() {} 

#[derive(Message,Clone)]
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

// 实现Actor
impl Actor for MarkdownSyncService {
    type Context = Context<Self>;
}
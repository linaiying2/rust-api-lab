use std::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct ConfigManager {
    settings: RwLock<HashMap<String, String>>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            settings: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn set(&self, key: &str, value: &str) {
        let mut settings = self.settings.write().unwrap();
        settings.insert(key.to_string(), value.to_string());
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        let settings = self.settings.read().unwrap();
        settings.get(key).cloned()
    }
    
    pub async fn start_watch(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                manager.reload_config().await;
            }
        });
    }
    
    async fn reload_config(&self) {
        // 从文件或环境变量重新加载配置
        println!("重新加载配置...");
    }
}

// 实现Clone trait
impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        let settings = self.settings.read().unwrap();
        Self {
            settings: RwLock::new(settings.clone()),
        }
    }
}
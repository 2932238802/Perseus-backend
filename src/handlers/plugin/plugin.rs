use crate::models::plugin::Plugin;
use axum::Json;
use std::fs;

use std::env;

pub async fn get_plugins() -> Json<Vec<Plugin>> {
    let file_path = env::var("PLUGIN_FILE_PATH")
        .unwrap_or_else(|_| "./data/plugins.json".to_string());
    let file_content = fs::read_to_string(&file_path)
        .expect(&format!("读取文件失败，路径: {}", file_path));
    let plugins: Vec<Plugin> = serde_json::from_str(&file_content).unwrap();
    Json(plugins)
}

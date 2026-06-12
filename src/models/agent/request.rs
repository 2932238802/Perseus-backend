use serde::Deserialize;

/// 增肌 request的请求
#[derive(Deserialize)]
pub struct AddAgentRequest {
    pub provider_name: String,
    pub base_url: String,
    pub models: Vec<String>,
    pub api_key: String,
}

/// 获取模型列表的请求
#[derive(Deserialize)]
pub struct ListModelsRequest {
    pub base_url: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub provider_name: String,
    pub model: String,
}

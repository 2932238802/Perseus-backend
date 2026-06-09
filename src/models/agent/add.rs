use serde::Deserialize;

/// 增肌 request的请求
#[derive(Deserialize)]
pub struct AddAgentRequest {
    pub provider_name: String,
    pub base_url: String,
    pub models: Vec<String>,
    pub api_key: String,
}

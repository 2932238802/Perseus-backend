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

/// 删除厂商/模型的请求: model 为空则删除整个厂商, 非空则只删该模型
#[derive(Deserialize)]
pub struct DeleteAgentRequest {
    pub provider_name: String,
    #[serde(default)]
    pub model: Option<String>,
}

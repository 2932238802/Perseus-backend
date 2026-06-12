use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{models::agent::request::ListModelsRequest, utils::extract::auth_extractor::AuthUser};

pub async fn list_models(
    AuthUser(user): AuthUser,
    Json(request): Json<ListModelsRequest>,
) -> impl IntoResponse {
    let url = format!("{}/models", request.base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let res = client.get(&url).bearer_auth(&request.api_key).send().await;
    let res = match res {
        Ok(response) => response,
        Err(e) => {
            eprintln!("无法连接该厂商接口 error: {:?}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "success": false, "message": "无法连接该厂商接口" })),
            );
        }
    };

    let body: serde_json::Value = match res.json().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("厂商解析失败 error: {:?}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!(
                    {
                        "success":false,
                        "message":"厂商解析失败"
                    }
                )),
            );
        }
    };

    // {
    //   "data": [
    //     { "id": "deepseek-chat",     "object": "model" },
    //     { "  id": "deepseek-reasoner", "object": "model" }
    //   ]
    // }
    let models: Vec<String> = body["data"]
        .as_array()
        .map(|arr| {
            // 这里是 option 解包
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "success" : true,
            "models" : models
        })),
    )
}

use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::{models::agent::request::ChatRequest, utils::extract::auth_extractor::AuthUser};

pub async fn chat(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(request): Json<ChatRequest>,
) -> impl IntoResponse {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT base_url,api_key FROM ai_provider WHERE user_id = $1 AND provider_name = $2",
    )
    .bind(user.sub)
    .bind(&request.provider_name)
    .fetch_optional(&pool)
    .await;
    let (base_url, api_key) = match row {
        Ok(Some(v)) => v,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "未找到该厂商配置" })),
            );
        }
        Err(e) => {
            eprintln!("chat 查询配置失败: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "服务器错误" })),
            );
        }
    };

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let payload = json!({
        "model": request.model,
        "messages": [ { "role": "user", "content": request.message } ]
    });
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .bearer_auth(&api_key)
        .json(&payload)
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "success": false, "message": "无法连接 AI 接口" })),
            );
        }
    };
    let body: serde_json::Value = match res.json().await {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "success": false, "message": "AI 响应解析失败" })),
            );
        }
    };
    let reply = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    (
        StatusCode::OK,
        Json(json!({ "success": true, "reply": reply })),
    )
}

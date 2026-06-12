use async_stream::stream;

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response, Sse}, // ← 改动1：加了 Response、Sse
};
use futures::StreamExt;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::{models::agent::request::ChatRequest, utils::extract::auth_extractor::AuthUser};

pub async fn chat(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(request): Json<ChatRequest>,
) -> Response {
    // ← 改动2：impl IntoResponse 改成 Response
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
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("chat 查询配置失败: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "服务器错误" })),
            )
                .into_response();
        }
    };

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let payload = json!({
        "model": request.model,
        "messages": [ { "role": "user", "content": request.message } ],
        "stream": true
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
            )
                .into_response();
        }
    };

    let mut stream = res.bytes_stream();
    let sse_stream = stream! {
        'outer: while let Some(chunk) = stream.next().await {
            let chunk_ok = match chunk {
                Ok(c) => c,
                Err(_) => break 'outer,
            };
            let text = String::from_utf8_lossy(&chunk_ok);
            for line in text.lines() {
                let line = line.trim();
                if !line.starts_with("data:") {
                    continue;
                }
                let payload = line["data:".len()..].trim();
                if payload == "[DONE]" {
                    break 'outer;
                }
                let v: serde_json::Value = match serde_json::from_str(payload) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let piece = v["choices"][0]["delta"]["content"]
                    .as_str()
                    .unwrap_or("");
                if !piece.is_empty() {
                    yield Ok::<_, std::convert::Infallible>(
                        axum::response::sse::Event::default()
                            .json_data(json!({ "d": piece }))
                            .unwrap(),
                    );
                }
            }
        }
    };

    Sse::new(sse_stream).into_response()
}

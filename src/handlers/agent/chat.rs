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

fn extract_error_message(body: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let msg = v["error"]["message"]
        .as_str()
        .or_else(|| v["error"].as_str())
        .or_else(|| v["message"].as_str())?;
    Some(msg.to_string())
}



/**
 * 交流
 */
pub async fn chat(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(request): Json<ChatRequest>,
) -> Response {
    // impl IntoResponse 改成 Response
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

    // 上游(如 ark)返回非 2xx 时, 把具体错误透传给前端, 而不是默默流空导致"发消息没回"
    let upstream_status = res.status().as_u16();
    if !res.status().is_success() {
        let body = res.text().await.unwrap_or_default();
        let err_msg = extract_error_message(&body)
            .unwrap_or_else(|| format!("AI 接口返回错误 (HTTP {})", upstream_status));
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({ "success": false, "message": err_msg })),
        )
            .into_response();
    }

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

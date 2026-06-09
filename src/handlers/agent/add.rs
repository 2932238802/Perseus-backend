use crate::{models::agent::add::AddAgentRequest, utils::extract::auth_extractor::AuthUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

pub async fn add_agent(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(request): Json<AddAgentRequest>,
) -> impl IntoResponse {
    let provider_id_res = sqlx::query_scalar(
        "INSERT INTO ai_provider (user_id, provider_name, base_url, api_key)
         VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(user.sub)
    .bind(&request.provider_name)
    .bind(&request.base_url)
    .bind(&request.api_key)
    .fetch_one(&pool)
    .await;

    let provider_id: i32 = match provider_id_res {
        OK(id) => id,
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            return (
                StatusCode::CONFLICT,
                Json(json!(
                    {
                        "success" : false,
                        "message" : "该厂商的配置 已经存在了"
                    }
                )),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success" : false,
                    "message" : "服务器错误"
                })),
            );
        }
    };

    for model_name in &request.models {
        sqlx::query("INSERT INTO ai_model (provider_id, model_name) VALUES ($1, $2)")
            .bind(provider_id)
            .bind(model_name)
            .execute(&pool)
            .await
            .unwrap();
    }

    (
        StatusCode::OK,
        Json(serde_json::json!(
            {
                "success": true,
                "message": "添加成功"
            }
        )),
    )
}

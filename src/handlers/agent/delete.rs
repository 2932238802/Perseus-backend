use crate::{models::agent::request::DeleteAgentRequest, utils::extract::auth_extractor::AuthUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

/// 删除厂商或模型
/// - request.model 为空: 删除整个厂商 (ai_model 通过外键级联删除)
/// - request.model 非空: 只删除该厂商下的指定模型
pub async fn delete_agent(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(request): Json<DeleteAgentRequest>,
) -> impl IntoResponse {
    let provider_id = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM ai_provider WHERE user_id = $1 AND provider_name = $2",
    )
    .bind(user.sub)
    .bind(&request.provider_name)
    .fetch_optional(&pool)
    .await;

    let provider_id = match provider_id {
        Ok(Some(id)) => id,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "未找到该厂商" })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "服务器错误" })),
            );
        }
    };

    if let Some(model) = &request.model {
        if model.trim().is_empty() {
            return delete_provider(&pool, provider_id).await;
        }
        let res = sqlx::query("DELETE FROM ai_model WHERE provider_id = $1 AND model_name = $2")
            .bind(provider_id)
            .bind(model)
            .execute(&pool)
            .await;
        match res {
            Ok(r) if r.rows_affected() > 0 => {
                (StatusCode::OK, Json(json!({ "success": true, "message": "模型已删除" })))
            }
            Ok(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "message": "未找到该模型" })),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "服务器错误" })),
            ),
        }
    } else {
        delete_provider(&pool, provider_id).await
    }
}

async fn delete_provider(
    pool: &PgPool,
    provider_id: i32,
) -> (StatusCode, Json<serde_json::Value>) {
    let res = sqlx::query("DELETE FROM ai_provider WHERE id = $1")
        .bind(provider_id)
        .execute(pool)
        .await;
    match res {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true, "message": "厂商已删除" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "message": "服务器错误" })),
        ),
    }
}

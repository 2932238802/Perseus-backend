use crate::utils::jwt::create_token;
use crate::models::auth::RegisterRequest;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    let hash_pd = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).unwrap();
    let res = sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
    )
    .bind(&req.username)
    .bind(&hash_pd)
    .fetch_one(&pool)
    .await;
    match res {
        Ok(user_id) => match create_token(user_id, &req.username) {
            Ok(token) => (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "message": "注册成功",
                    "token": token
                })),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "message": "token 签发失败" })),
            ),
        },
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "success": false, "message": "用户名已存在" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "message": "服务器错误" })),
        ),
    }
}

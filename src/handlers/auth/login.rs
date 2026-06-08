use crate::utils::jwt::create_token;
use crate::models::auth::LoginRequest;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;


/// 注册 回调
pub async fn login(
    State(pool): State<PgPool>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let row = sqlx::query_as::<_, (i32, String)>("SELECT id, password FROM users WHERE username = $1")
        .bind(&request.username)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some((user_id, hash))) => match bcrypt::verify(&request.password, &hash) {
            Ok(true) => {
                // 密码正确 -> 签发 token
                match create_token(user_id, &request.username) {
                    Ok(token) => (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "success": true,
                            "message": "登录成功",
                            "token": token
                        })),
                    ),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "success": false, "message": "token 签发失败" })),
                    ),
                }
            }
            Ok(false) => (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "success": false, "message": "用户名或密码错误" })),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "message": "服务器错误" })),
            ),
        },
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "success": false, "message": "用户名或密码错误" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "message": "服务器错误" })),
        ),
    }
}

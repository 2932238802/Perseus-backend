use crate::utils::extract::auth_extractor::AuthUser;
use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn me(AuthUser(claims): AuthUser) -> impl IntoResponse {
    Json(json!(
        {
            "success" : true,
            "message" : "自动登录成功",
            "username" : &claims.username,
            "user_id" : &claims.sub
        }
    ))
}

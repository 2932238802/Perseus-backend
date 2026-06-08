use axum::{Json, response::IntoResponse};
use serde_json::json;
use crate::utils::extract::auth_extractor::AuthUser;

pub async fn me(AuthUser(claims): AuthUser) -> impl IntoResponse {
    Json(json!(
        {
            "success" : true,
            "message" : "自动登录成功",
        }
    ))    
}

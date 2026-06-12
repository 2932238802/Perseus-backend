use crate::utils::extract::auth_extractor::AuthUser;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn list_providers(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    // 先从数据库找到对应的 provider models
    // query_as 可以映射成 自己想要的数据
    let res = sqlx::query_as::<_, (String, String)>(
        "
        SELECT p.provider_name, m.model_name
        FROM ai_provider p
        JOIN ai_model m ON m.provider_id = p.id
        WHERE p.user_id = $1
        ORDER BY p.provider_name
        ",
    )
    .bind(user.sub)
    .fetch_all(&pool)
    .await;
    let rows_from_res = match res {
        Ok(r) => r,
        Err(e) => {
            eprintln!("内部服务器错误 error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message" : "内部服务器错误",
                    "success" : false,
                })),
            );
        }
    };
    let mut group_by_provides_to_models: HashMap<String, Vec<String>> = HashMap::new();
    for (provides, model) in rows_from_res {
        group_by_provides_to_models
            .entry(provides)
            .or_default()
            .push(model);
    }
    let provider: Vec<_> = group_by_provides_to_models
        .into_iter()
        .map(|(name, vec)| {
            json!(
                {
                    "provider_name":name,
                    "models":vec,
                }
            )
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "success" : true,
            "providers" : provider
        })),
    )
}

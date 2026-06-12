use crate::handlers::agent::{
    add::add_agent, chat::chat, list_models::list_models, list_providers::list_providers,
};
use axum::{Router, routing::post};
use sqlx::PgPool;

// Agent 相关路由
pub fn agent_router() -> Router<PgPool> {
    Router::new()
        .route("/LosAngelous/api/agent/add", post(add_agent))
        .route("/LosAngelous/api/agent/list_models", post(list_models))
        .route(
            "/LosAngelous/api/agent/list_providers",
            post(list_providers),
        )
        .route("/LosAngelous/api/agent/chat", post(chat))
}

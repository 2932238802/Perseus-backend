use crate::handlers::auth::{login::login, me::me, register::register};
use axum::{
    Router,
    routing::{Route, post},
};
use sqlx::PgPool;

// 注册 逻辑
// 函数 默认 蛇形
// 常量是大写
pub fn auth_router() -> Router<PgPool> {
    Router::new()
        .route("/LosAngelous/api/register", post(register))
        .route("/LosAngelous/api/login", post(login))
        .route("/LosAngelous/api/me", post(me))
}

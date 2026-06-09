use crate::handlers::plugin::plugin::get_plugins;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn plugin_router() -> Router<PgPool> {
    Router::new().route("/LosAngelous/api/plugins", get(get_plugins))
}

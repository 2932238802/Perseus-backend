use crate::handlers::plugin::get_plugins;
use axum::{Router, routing::get};

pub fn plugin_router() -> Router {
    Router::new().route("/LosAngelous/api/plugins", get(get_plugins))
}

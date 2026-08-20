use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未在 .env 中配置");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("无法连接到 PostgreSQL, 请检查 DATABASE_URL 与数据库是否启动");
    let app = Router::new()
        .route("/", get(|| async { "~ LosAngelous ~" }))
        .nest_service("/downloads", ServeDir::new("downloads"))
        .merge(routes::plugin::plugin_router())
        .merge(routes::auth::auth_router())
        .merge(routes::agent::agent_router())
        .with_state(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

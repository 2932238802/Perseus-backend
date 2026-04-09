use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "~ LosAngelous ~" }))
        .nest_service("/downloads", ServeDir::new("downloads"))
        .merge(routes::plugin::plugin_router());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Perseus Backend is running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

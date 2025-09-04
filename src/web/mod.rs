use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub mod api;
pub mod handlers;

use crate::core::setup_db;

pub async fn start_server() {
    if let Err(e) = setup_db(false) {
        eprintln!("Failed to initialize database: {e}");
        std::process::exit(1);
    }

    let app = Router::new()
        // API routes
        .nest("/api", api::routes())
        // Serve built frontend (from frontend/dist)
        .fallback_service(ServeDir::new("frontend/dist"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}


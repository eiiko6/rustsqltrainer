use std::net::SocketAddr;

mod embed;
use embed::app_router;

pub mod api;
pub mod handlers;

use crate::core::setup_db;

pub async fn start_server() {
    if let Err(e) = setup_db(false) {
        eprintln!("Failed to initialize database: {e}");
        std::process::exit(1);
    }

    let app = app_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("server error");
}

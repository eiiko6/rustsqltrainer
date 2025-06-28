use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

pub mod handlers;
use crate::core::setup_db;

pub async fn start_server() {
    if let Err(e) = setup_db(false) {
        eprintln!("Failed to initialize database: {e}");
        std::process::exit(1);
    }

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/exercise/{id}", get(handlers::exercise))
        .route("/submit", post(handlers::submit));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let url = format!("http://{}", addr);

    if webbrowser::open(&url).is_ok() {
        println!("Opened in your browser");
    }
    println!("Listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

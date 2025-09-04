use axum::{
    Router,
    body::Body,
    http::{StatusCode, Uri},
    response::Response,
    routing::get,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/dist"] // path to built Vue app
struct Assets;

// Serve embedded files
pub async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/').replace('\\', "/"); // normalize

    let file = if path.is_empty() { "index.html" } else { &path };

    match Assets::get(file) {
        Some(content) => {
            let body = Body::from(content.data.into_owned());
            let mime = mime_guess::from_path(file).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => {
            // fallback to index.html for SPA routes
            match Assets::get("index.html") {
                Some(content) => {
                    let body = Body::from(content.data.into_owned());
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/html")
                        .body(body)
                        .unwrap()
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            }
        }
    }
}

pub fn app_router() -> Router {
    Router::new()
        .nest("/api", crate::web::api::routes())
        .route("/", axum::routing::get(static_handler)) // root route
        .route("/{*path}", get(static_handler)) // serve Vue app
}

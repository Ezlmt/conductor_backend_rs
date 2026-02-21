use axum::{routing::get, Router};

async fn ping() -> &'static str {
    "ok"
}

pub fn create_app() -> Router {
    Router::new().route("/ping", get(ping))
}

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::{
    routes::user::users_routes,
    state::AppState,
};

async fn ping() -> &'static str {
    "ok"
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .merge(users_routes(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::span!(
                        Level::INFO,
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                )
                }),
        )
}

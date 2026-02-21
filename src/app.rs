use axum::{routing::get, Router};
use sqlx::PgPool;

async fn ping() -> &'static str {
    "ok"
}

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .with_state(pool)
}

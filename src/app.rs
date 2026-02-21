use axum::{routing::get, Router};

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
}

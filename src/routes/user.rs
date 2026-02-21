use axum::{routing::post, Router};
use crate::{handlers::user::{login_user, register_user}, state::AppState};

pub fn users_routes(state: AppState) -> Router {
    Router::new()
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        .with_state(state)
}

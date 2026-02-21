use axum::{routing::post, Router};
use crate::{handlers::user::register_user, state::AppState};

pub fn users_routes(state: AppState) -> Router {
    Router::new()
        .route("/users/register", post(register_user))
        .with_state(state)
}

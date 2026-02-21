use axum::{extract::State, Json};
use sqlx::query_as;
use tracing::info;

use crate::{
    models::user::{RegisterRequest, User},
    state::AppState,
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, String> {
    let user = query_as!(
        User,
        r#"
        INSERT INTO users (email, password, name)
        VALUES ($1, $2, $3)
        RETURNING id, email, name
        "#,
        payload.email,
        payload.password,
        payload.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| format!("db error: {}", e))?;

    info!("Regist successfully! User: {}", user.name);
    Ok(Json(user))
}

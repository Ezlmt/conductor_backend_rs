use axum::{extract::State, Json};
use bcrypt::{hash, DEFAULT_COST};
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
    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| format!("hash error: {}", e))?;
    let user = query_as!(
        User,
        r#"
        INSERT INTO users (email, password, name)
        VALUES ($1, $2, $3)
        RETURNING id, email, name
        "#,
        payload.email,
        hashed_password,
        payload.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| format!("db error: {}", e))?;

    info!(email = %payload.email, "register user");
    Ok(Json(user))
}

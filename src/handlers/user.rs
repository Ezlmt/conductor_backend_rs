use axum::{extract::State, Json};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::query_as;
use tracing::info;

use crate::{
    models::{auth::{Claims, LoginRequest, LoginResponse}, user::{RegisterRequest, User}},
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

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, String> {
    // 1. query user
    let row = sqlx::query!(
        r#"
        SELECT id, email, password
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| format!("db error: {}", e))?
    .ok_or_else(|| "invalid credentials".to_string())?;

    // 2. check password
    let ok = verify(&payload.password, &row.password)
        .map_err(|_| "invalid credcentials".to_string())?;
    if !ok {
        return Err("invalid credcentials".to_string());
    }

    // 3. Generate JWT
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: row.id,
        email: row.email,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| format!("jwt error: {}", e))?;

    Ok(Json(LoginResponse { token}))
}

#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn password_hash_and_verify_works() {
        let password = "123456";

        let hashed = hash(password, DEFAULT_COST).unwrap();

        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrong", &hashed).unwrap());
    }
}

use argon2::{Argon2, PasswordHasher};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use password_hash::{SaltString, rand_core::OsRng};

use crate::{
    models::user::{CreateUserRequest, User},
    services::user_service,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    if payload.email.trim().is_empty() || payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "email or password invalid".into()));
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error while generating hash".into(),
            )
        })?
        .to_string();

    let user = user_service::create_user(&state.db, &payload.email, &password_hash)
        .await
        .map_err(|e| {
            eprintln!("db error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error while creating user".into(),
            )
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}

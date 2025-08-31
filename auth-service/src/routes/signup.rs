use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::domain::AuthAPIError;

use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Create a new `User` instance using data in the `request`
    let user = User::new(email, password, request.require_2fa);

    let mut user_store = state.user_store.write().await;

     // TODO: early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.get_user(&user.email).is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // TODO: instead of using unwrap, early return AuthAPIError::UnexpectedError if add_user() fails.
    if user_store.add_user(user).is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub require_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    AppState,
    errors::AppError,
    models::{AuthResponse, LoginRequest, RegisterRequest},
    services,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(register_request): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let auth_response = services::auth::register_account(
        &state.database_pool,
        register_request,
        &state.token_signing_secret,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(auth_response)))
}

async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let auth_response = services::auth::authenticate_account(
        &state.database_pool,
        login_request,
        &state.token_signing_secret,
    )
    .await?;

    Ok(Json(auth_response))
}

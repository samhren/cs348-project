use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{AppState, errors::AppError, models::SessionClaims};

pub struct AuthenticatedUser(pub SessionClaims);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bearer_token = parts
            .headers
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .and_then(|header_value| header_value.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        let token_data = decode::<SessionClaims>(
            bearer_token,
            &DecodingKey::from_secret(state.token_signing_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthenticatedUser(token_data.claims))
    }
}

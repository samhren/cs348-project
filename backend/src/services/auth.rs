use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    models::{AuthResponse, LoginRequest, RegisterRequest, SessionClaims, User},
};

use super::input::{validate_email, validate_password, validate_username};

type UserWithPasswordHash = (
    uuid::Uuid,
    String,
    String,
    String,
    String,
    chrono::DateTime<Utc>,
);

pub async fn register_account(
    database_pool: &PgPool,
    register_request: RegisterRequest,
    token_signing_secret: &str,
) -> Result<AuthResponse, AppError> {
    let username = validate_username(&register_request.username)?;
    let email = validate_email(&register_request.email)?;
    let password = validate_password(&register_request.password)?;
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|error| AppError::BadRequest(error.to_string()))?;
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *transaction)
        .await?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, role)
         VALUES ($1, $2, $3, 'user')
         RETURNING id, username, email, role, created_at",
    )
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(&mut *transaction)
    .await?;

    let token = create_session_token(&user, token_signing_secret)?;
    transaction.commit().await?;

    Ok(AuthResponse { token, user })
}

pub async fn authenticate_account(
    database_pool: &PgPool,
    login_request: LoginRequest,
    token_signing_secret: &str,
) -> Result<AuthResponse, AppError> {
    let username = validate_username(&login_request.username)?;
    let password = validate_password(&login_request.password)?;

    let stored_user = sqlx::query_as::<_, UserWithPasswordHash>(
        "SELECT id, username, email, password_hash, role, created_at FROM users WHERE username = $1",
    )
    .bind(&username)
    .fetch_optional(database_pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let password_matches =
        bcrypt::verify(password, &stored_user.3).map_err(|_| AppError::Unauthorized)?;

    if !password_matches {
        return Err(AppError::Unauthorized);
    }

    let user = User {
        id: stored_user.0,
        username: stored_user.1,
        email: stored_user.2,
        role: stored_user.4,
        created_at: stored_user.5,
    };
    let token = create_session_token(&user, token_signing_secret)?;

    Ok(AuthResponse { token, user })
}

fn create_session_token(user: &User, token_signing_secret: &str) -> Result<String, AppError> {
    let expires_at = (Utc::now() + Duration::hours(24)).timestamp() as u64;
    let claims = SessionClaims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        role: user.role.clone(),
        exp: expires_at,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(token_signing_secret.as_bytes()),
    )
    .map_err(|error| AppError::BadRequest(error.to_string()))
}

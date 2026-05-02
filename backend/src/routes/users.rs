use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use uuid::Uuid;

use crate::{
    AppState,
    auth::AuthenticatedUser,
    errors::AppError,
    models::{CreateUserRequest, UpdateUserRequest, User, UserSearchFilters},
    services,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/roles", get(list_roles))
        .route("/", get(list_users).post(create_user))
        .route("/:id", axum::routing::put(update_user).delete(delete_user))
}

async fn list_roles(
    State(state): State<AppState>,
    _authenticated_user: AuthenticatedUser,
) -> Result<Json<Vec<String>>, AppError> {
    let roles = services::users::list_roles(&state.database_pool).await?;
    Ok(Json(roles))
}

async fn list_users(
    State(state): State<AppState>,
    _authenticated_user: AuthenticatedUser,
    Query(search_filters): Query<UserSearchFilters>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = services::users::list_users(&state.database_pool, &search_filters).await?;
    Ok(Json(users))
}

async fn create_user(
    State(state): State<AppState>,
    authenticated_user: AuthenticatedUser,
    Json(create_user_request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    ensure_admin_access(&authenticated_user)?;

    let user = services::users::create_user(&state.database_pool, create_user_request).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    State(state): State<AppState>,
    authenticated_user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Json(update_user_request): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    ensure_admin_access(&authenticated_user)?;

    let user =
        services::users::update_user(&state.database_pool, user_id, update_user_request).await?;
    Ok(Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    authenticated_user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    ensure_admin_access(&authenticated_user)?;
    services::users::delete_user(&state.database_pool, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

fn ensure_admin_access(authenticated_user: &AuthenticatedUser) -> Result<(), AppError> {
    if authenticated_user.0.role != "admin" {
        return Err(AppError::Unauthorized);
    }

    Ok(())
}

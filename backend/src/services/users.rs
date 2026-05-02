use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{CreateUserRequest, UpdateUserRequest, User, UserSearchFilters},
};

use super::input::{
    sanitize_optional_search_term, validate_email, validate_optional_role, validate_password,
    validate_role, validate_username,
};

pub async fn list_users(
    database_pool: &PgPool,
    search_filters: &UserSearchFilters,
) -> Result<Vec<User>, AppError> {
    let role = validate_optional_role(&search_filters.role)?;
    let username = sanitize_optional_search_term(&search_filters.username)?;
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *transaction)
        .await?;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT id, username, email, role, created_at FROM users WHERE 1=1",
    );

    if let Some(role) = &role {
        query_builder.push(" AND role = ").push_bind(role);
    }

    if let Some(username) = &username {
        query_builder
            .push(" AND username ILIKE ")
            .push_bind(format!("%{username}%"));
    }

    query_builder.push(" ORDER BY created_at DESC");

    let users = query_builder
        .build_query_as::<User>()
        .fetch_all(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(users)
}

pub async fn create_user(
    database_pool: &PgPool,
    create_user_request: CreateUserRequest,
) -> Result<User, AppError> {
    let username = validate_username(&create_user_request.username)?;
    let email = validate_email(&create_user_request.email)?;
    let password = validate_password(&create_user_request.password)?;
    let role = validate_role(&create_user_request.role)?;
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|error| AppError::BadRequest(error.to_string()))?;
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *transaction)
        .await?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, role)
         VALUES ($1, $2, $3, $4)
         RETURNING id, username, email, role, created_at",
    )
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .bind(&role)
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(user)
}

pub async fn update_user(
    database_pool: &PgPool,
    user_id: Uuid,
    update_user_request: UpdateUserRequest,
) -> Result<User, AppError> {
    let username = validate_username(&update_user_request.username)?;
    let email = validate_email(&update_user_request.email)?;
    let role = validate_role(&update_user_request.role)?;
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
        .execute(&mut *transaction)
        .await?;

    let current_role: Option<(String,)> =
        sqlx::query_as("SELECT role FROM users WHERE id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *transaction)
            .await?;

    let Some((current_role,)) = current_role else {
        return Err(AppError::NotFound);
    };

    if current_role == "admin" && role != "admin" {
        ensure_another_admin_exists(&mut transaction, user_id).await?;
    }

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, email = $2, role = $3
         WHERE id = $4
         RETURNING id, username, email, role, created_at",
    )
    .bind(&username)
    .bind(&email)
    .bind(&role)
    .bind(user_id)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(AppError::NotFound)?;

    transaction.commit().await?;

    Ok(user)
}

pub async fn delete_user(database_pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
        .execute(&mut *transaction)
        .await?;

    let current_role: Option<(String,)> =
        sqlx::query_as("SELECT role FROM users WHERE id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *transaction)
            .await?;

    let Some((current_role,)) = current_role else {
        return Err(AppError::NotFound);
    };

    if current_role == "admin" {
        ensure_another_admin_exists(&mut transaction, user_id).await?;
    }

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn list_roles(database_pool: &PgPool) -> Result<Vec<String>, AppError> {
    let roles: Vec<(String,)> = sqlx::query_as("SELECT DISTINCT role FROM users ORDER BY role")
        .fetch_all(database_pool)
        .await?;

    Ok(roles.into_iter().map(|(role,)| role).collect())
}

async fn ensure_another_admin_exists(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
) -> Result<(), AppError> {
    let other_admins: Vec<(Uuid,)> =
        sqlx::query_as("SELECT id FROM users WHERE role = 'admin' AND id <> $1 FOR SHARE")
            .bind(user_id)
            .fetch_all(&mut **transaction)
            .await?;

    if other_admins.is_empty() {
        return Err(AppError::BadRequest(
            "cannot remove the last admin".to_string(),
        ));
    }

    Ok(())
}

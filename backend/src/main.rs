mod auth;
mod errors;
mod models;
mod routes;
mod services;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub database_pool: sqlx::PgPool,
    pub token_signing_secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let token_signing_secret =
        env::var("JWT_SECRET").unwrap_or_else(|_| "changeme_secret_key".to_string());

    let database_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    ensure_default_admin_account_exists(&database_pool).await?;

    let app_state = AppState {
        database_pool,
        token_signing_secret,
    };

    let app = Router::new()
        .route("/health", get(health))
        .nest("/auth", routes::auth::router())
        .nest("/users", routes::users::router())
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    tracing::info!("listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn ensure_default_admin_account_exists(database_pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let mut transaction = database_pool.begin().await?;

    sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
        .execute(&mut *transaction)
        .await?;

    let admin_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'admin'")
        .fetch_one(&mut *transaction)
        .await?;

    if admin_count == 0 {
        let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;
        sqlx::query(
            "INSERT INTO users (username, email, password_hash, role)
             VALUES ('admin', 'admin@gmail.com', $1, 'admin')",
        )
        .bind(password_hash)
        .execute(&mut *transaction)
        .await?;

        tracing::info!("Created default admin user: admin / admin123");
    }

    transaction.commit().await?;

    Ok(())
}

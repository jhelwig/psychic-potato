use anyhow::Result;
use axum_session::{
    SessionConfig,
    SessionStore,
};
use axum_session_auth::AuthConfig;
use axum_session_sqlx::SessionSqlitePool;
use log::LevelFilter;
use sqlx::{
    Executor,
    sqlite::SqlitePoolOptions,
};
use tokio::signal;
use uuid::Uuid;

pub mod app;
pub mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:league.db?mode=rwc".to_string());
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        unsafe {
            std::env::set_var("RUST_LIB_BACKTRACE", "1");
        }
    }

    let db_pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_slow_level(LevelFilter::Warn)
        .acquire_slow_threshold(std::time::Duration::from_millis(50))
        .acquire_timeout(std::time::Duration::from_millis(250))
        .test_before_acquire(true)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                conn.execute("PRAGMA foreign_keys = on;").await?;

                Ok(())
            })
        })
        .connect(&db_connection_str)
        .await?;

    let session_config = SessionConfig::default().with_table_name("sessions");
    let session_store = SessionStore::<SessionSqlitePool>::new(
        Some(SessionSqlitePool::from(db_pool.clone())),
        session_config,
    )
    .await?;

    let auth_config = AuthConfig::<Uuid>::default().with_anonymous_user_id(None);

    sqlx::migrate!().run(&db_pool).await?;

    let app_state = app::AppState {
        db_pool,
    };
    let app = app::build(app_state, session_store, auth_config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000").await?;
    println!("Server running on http://127.0.0.1:9000/");

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.map_err(Into::into)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

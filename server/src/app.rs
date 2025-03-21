use axum::{
    Json,
    Router,
    body::HttpBody,
    extract::{
        FromRef,
        FromRequestParts,
    },
    http::{
        HeaderValue,
        StatusCode,
        header::CONTENT_LENGTH,
    },
    response::{
        IntoResponse,
        Response,
    },
};
use axum_session::{
    SessionLayer,
    SessionStore,
};
use axum_session_auth::AuthConfig;
use axum_session_sqlx::SessionSqlitePool;
use serde_json::json;
use sqlx::{
    Sqlite,
    SqlitePool,
    Transaction,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    decompression::RequestDecompressionLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::{
    app::auth::AppAuthSessionLayer,
    error::AppError,
};

pub mod auth;
pub mod classes;
pub mod export;
pub mod leagues;
pub mod matches;
pub mod shots;
pub mod strings;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

pub struct DbTransaction<'a>(Transaction<'a, Sqlite>);

impl<S> FromRequestParts<S> for DbTransaction<'_>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let conn = app_state.db_pool.begin().await?;

        Ok(Self(conn))
    }
}

pub fn build(
    app_state: AppState,
    session_store: SessionStore<SessionSqlitePool>,
    auth_config: AuthConfig<Uuid>,
) -> Router {
    let service_builder = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(RequestDecompressionLayer::new())
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::overriding(CONTENT_LENGTH, content_length_from_response))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(SessionLayer::new(session_store))
        .layer(AppAuthSessionLayer::new(Some(app_state.db_pool.clone())).with_config(auth_config));

    Router::new()
        .nest("/league", leagues::router(app_state.clone()))
        .nest("/user", auth::routes::router(app_state.clone()))
        .layer(service_builder)
        .with_state(app_state)
        .fallback(handler_404)
}

fn content_length_from_response<B>(response: &Response<B>) -> Option<HeaderValue>
where
    B: HttpBody,
{
    response
        .body()
        .size_hint()
        .exact()
        .map(|size| HeaderValue::from_str(&size.to_string()).unwrap())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({"message": "Not found"}))).into_response()
}

use anyhow::Context;
use axum::{
    Json,
    Router,
    routing::{
        get,
        post,
    },
};
use log::{
    error,
    info,
};
use shared_types::response::User;
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        DbTransaction,
        auth::{
            AppAuthSession,
            AuthError,
            AuthenticatedUser,
        },
    },
    error::{
        AppError,
        HttpResponse,
    },
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_current_user))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
        .with_state(app_state)
}

pub async fn get_current_user(
    AuthenticatedUser(auth_session): AuthenticatedUser,
) -> Result<Json<User>, AppError> {
    let Some(user) = auth_session.current_user else {
        return Err(AuthError::NotLoggedIn).context(HttpResponse::Unauthorized).map_err(Into::into);
    };
    info!("Current user: {:?}", &user);

    Ok(Json(user.into()))
}

pub async fn login(
    DbTransaction(mut txn): DbTransaction<'_>,
    auth_session: AppAuthSession,
    Json(login_info): Json<shared_types::request::Login>,
) -> Result<Json<User>, AppError> {
    let username = &login_info.username;
    let maybe_user = sqlx::query_file_as!(
        crate::app::auth::User,
        "queries/auth/get_user_by_username.sql",
        username,
    )
    .fetch_optional(&mut *txn)
    .await?;

    let Some(user) = maybe_user else {
        info!("User not found: {}", username);
        return Err(AuthError::Unauthorized)
            .context(HttpResponse::Unauthorized)
            .map_err(Into::into);
    };

    if user
        .verify_password(&login_info.password)
        .context(AuthError::Unauthorized)
        .context(HttpResponse::Unauthorized)?
    {
        info!("User logged in: {}", username);
        auth_session.login_user(user.id);
        auth_session.remember_user(true);

        return Ok(Json(user.into()));
    }
    error!("Invalid password for user: {}", username);

    Err(AuthError::Unauthorized).context(HttpResponse::Unauthorized).map_err(Into::into)
}

pub async fn logout(AuthenticatedUser(auth_session): AuthenticatedUser) -> Result<(), AppError> {
    if auth_session.current_user.is_some() {
        auth_session.logout_user();

        Ok(())
    } else {
        Err(AuthError::Unauthorized).context(HttpResponse::Unauthorized).map_err(Into::into)
    }
}

pub async fn register(
    DbTransaction(mut txn): DbTransaction<'_>,
    auth_session: AppAuthSession,
    Json(register_info): Json<shared_types::request::RegisterUser>,
) -> Result<Json<User>, AppError> {
    let username = register_info.username.clone();
    let maybe_user = sqlx::query_file_as!(
        crate::app::auth::User,
        "queries/auth/get_user_by_username.sql",
        username
    )
    .fetch_optional(&mut *txn)
    .await?;

    if maybe_user.is_some() {
        return Err(AuthError::Unauthorized)
            .context(HttpResponse::Unauthorized)
            .map_err(Into::into);
    }

    let auth_user = crate::app::auth::User::new(&username, &register_info.password)?;
    let username = &auth_user.username;
    let password_hash = &auth_user.password_hash;

    sqlx::query_file!("queries/auth/create_user.sql", auth_user.id, username, password_hash,)
        .execute(&mut *txn)
        .await?;
    auth_session.login_user(auth_user.id);

    txn.commit().await?;
    Ok(Json(auth_user.into()))
}

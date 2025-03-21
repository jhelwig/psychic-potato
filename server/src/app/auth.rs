use anyhow::{
    Context,
    Result,
    anyhow,
};
use argon2::Argon2;
use async_trait::async_trait;
use axum::{
    extract::{
        FromRef,
        FromRequestParts,
    },
    http::request::Parts,
};
use axum_session_auth::{
    AuthSession,
    AuthSessionLayer,
    Authentication,
    HasPermission,
};
use axum_session_sqlx::SessionSqlitePool;
use password_hash::{
    PasswordHash,
    PasswordVerifier,
    SaltString,
    rand_core::OsRng,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

pub mod routes;

use crate::{
    app::AppState,
    error::{
        AppError,
        HttpResponse,
    },
};

pub type AppAuthSession = AuthSession<User, Uuid, SessionSqlitePool, SqlitePool>;
pub type AppAuthSessionLayer = AuthSessionLayer<User, Uuid, SessionSqlitePool, SqlitePool>;

pub struct AuthenticatedUser(pub AppAuthSession);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_session = AppAuthSession::from_request_parts(parts, state).await.map_err(
            |(status_code, error_body)| anyhow!("Auth failed ({status_code}): {error_body}"),
        )?;

        if auth_session.current_user.is_some() {
            Ok(Self(auth_session))
        } else {
            Err(AuthError::Unauthorized).context(HttpResponse::Unauthorized).map_err(Into::into)
        }
    }
}

pub struct UnauthenticatedUser(pub AppAuthSession);

impl<S> FromRequestParts<S> for UnauthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_session = AppAuthSession::from_request_parts(parts, state).await.map_err(
            |(status_code, error_body)| anyhow!("Auth failed ({status_code}): {error_body}"),
        )?;

        if auth_session.current_user.is_none() {
            Ok(Self(auth_session))
        } else {
            Err(AuthError::Unauthorized).context(HttpResponse::Unauthorized).map_err(Into::into)
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("DB connection pool not found")]
    NoDbConnectionPool,
    #[error("User not logged in")]
    NotLoggedIn,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("User not found: {userid}")]
    UserNotFound {
        userid: Uuid,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:        Uuid,
    pub username:  String,
    #[serde(skip_serializing)]
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<Self> {
        let id = Uuid::new_v4();

        let mut new_user = Self {
            id,
            username: username.to_owned(),
            password_hash: String::new(),
        };
        new_user.set_password(password)?;

        Ok(new_user)
    }

    pub fn set_password(&mut self, new_password: &str) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::generate(argon2, new_password, &salt)?;

        self.password_hash = password_hash.serialize().to_string();

        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool> {
        let password_hash = PasswordHash::new(&self.password_hash)?;
        let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];

        match password_hash.verify_password(algs, password) {
            Ok(_) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }
}

#[async_trait]
impl Authentication<User, Uuid, SqlitePool> for User {
    async fn load_user(userid: Uuid, pool: Option<&SqlitePool>) -> Result<User> {
        let Some(pool) = pool else {
            return Err(AuthError::NoDbConnectionPool.into());
        };

        let Some(user) = sqlx::query_file_as!(User, "queries/auth/get_user.sql", userid)
            .fetch_optional(pool)
            .await?
        else {
            return Err(AuthError::UserNotFound {
                userid,
            }
            .into());
        };

        Ok(user)
    }

    fn is_authenticated(&self) -> bool { true }

    fn is_active(&self) -> bool { true }

    fn is_anonymous(&self) -> bool { false }
}

#[async_trait]
impl HasPermission<SqlitePool> for User {
    async fn has(&self, _perm: &str, _pool: &Option<&SqlitePool>) -> bool { false }
}

impl From<User> for shared_types::response::User {
    fn from(value: User) -> Self {
        shared_types::response::User {
            id:       value.id,
            username: value.username,
        }
    }
}

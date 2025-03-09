use anyhow::Result;
use argon2::Argon2;
use async_trait::async_trait;
use axum_session_auth::{
    Authentication,
    HasPermission,
};
use password_hash::{
    PasswordHash,
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

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("DB connection pool not found")]
    NoDbConnectionPool,
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
    pub fn set_password(&mut self, new_password: &str) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::generate(argon2, new_password, &salt)?;

        self.password_hash = password_hash.serialize().to_string();

        Ok(())
    }
}

#[async_trait]
impl Authentication<User, Uuid, SqlitePool> for User {
    async fn load_user(userid: Uuid, pool: Option<&SqlitePool>) -> Result<User> {
        let Some(pool) = pool else {
            return Err(AuthError::NoDbConnectionPool.into());
        };

        // let Some(user) = sqlx::query_file_as!(User, "queries/auth/get_user.sql", userid)
        //     .fetch_optional(&pool)
        //     .await?
        // else {
        return Err(AuthError::UserNotFound {
            userid,
        }
        .into());
        // };
    }

    fn is_authenticated(&self) -> bool { true }

    fn is_active(&self) -> bool { true }

    fn is_anonymous(&self) -> bool { false }
}

#[async_trait]
impl HasPermission<SqlitePool> for User {
    async fn has(&self, _perm: &str, _pool: &Option<&SqlitePool>) -> bool { false }
}

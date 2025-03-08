use axum::{
    Json,
    Router,
    extract::Path,
    routing::{
        get,
        post,
    },
};
use shared_types::{
    request::ClassOperation,
    response::Class,
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        DbTransaction,
    },
    error::AppError,
};

#[derive(Debug, Error)]
pub enum MatchError {
    #[error("Class not found for league ({league_id}): {class_id}")]
    NotFound {
        league_id: Uuid,
        class_id:  Uuid,
    },
}

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_classes))
        .route("/{class_id}", get(get_class))
        .route("/operation", post(handle_class_operation))
        .with_state(app_state)
}

pub async fn list_classes(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
) -> Result<Json<Vec<Class>>, AppError> {
    let result = sqlx::query_file_as!(Class, "queries/classes/list_classes.sql", league_id)
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(result))
}

pub async fn get_class(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((league_id, class_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Class>, AppError> {
    let result = sqlx::query_file_as!(Class, "queries/classes/get_class.sql", league_id, class_id)
        .fetch_optional(&mut *txn)
        .await?;

    let Some(class) = result else {
        return Err(MatchError::NotFound {
            league_id,
            class_id,
        }
        .into());
    };

    Ok(Json(class))
}

pub async fn handle_class_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
    Json(operation): Json<ClassOperation>,
) -> Result<Json<Class>, AppError> {
    let class = match operation {
        ClassOperation::Create {
            name,
            description,
        } => {
            let id = Uuid::new_v4();
            sqlx::query_file_as!(
                Class,
                "queries/classes/create_class.sql",
                id,
                name,
                description,
                league_id,
            )
            .execute(&mut *txn)
            .await?;
            sqlx::query_file_as!(Class, "queries/classes/get_class.sql", league_id, id)
                .fetch_one(&mut *txn)
                .await?
        }
        ClassOperation::Delete {
            id,
        } => todo!(),
        ClassOperation::SetDescription {
            id,
            description,
        } => todo!(),
        ClassOperation::SetName {
            id,
            name,
        } => todo!(),
    };

    txn.commit().await?;
    Ok(Json(class))
}

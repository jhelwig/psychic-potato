use std::collections::HashMap;

use axum::{
    Json,
    Router,
    extract::Query,
    routing::{
        get,
        post,
    },
};
use ordered_float::OrderedFloat;
use shared_types::{
    request::ShooterOperation,
    response::Shooter,
};
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        DbTransaction,
    },
    error::{
        AppError,
        HttpResponse,
    },
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/operation", post(handle_shooter_operation))
        .route("/suggestions", get(suggestions))
        .with_state(app_state)
}

async fn index() -> Result<Json<Vec<Shooter>>, AppError> { Ok(Json(Vec::new())) }

async fn suggestions(
    DbTransaction(mut txn): DbTransaction<'_>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Shooter>>, AppError> {
    let Some(query_string) = query.get("query") else {
        return Err(HttpResponse::BadRequest {
            message: "Missing query parameter 'query'.".into(),
        }
        .into());
    };

    let mut shooters = sqlx::query_file_as!(Shooter, "queries/shooters/list_shooters.sql")
        .fetch_all(&mut *txn)
        .await?;

    shooters.sort_by_key(|shooter| OrderedFloat(strsim::jaro_winkler(query_string, &shooter.name)));
    shooters.reverse();

    Ok(Json(shooters))
}

async fn handle_shooter_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Json(operation): Json<ShooterOperation>,
) -> Result<Json<Shooter>, AppError> {
    let result = match operation {
        ShooterOperation::Create {
            name,
            default_class_id,
        } => {
            let id = Uuid::new_v4();
            sqlx::query_file!("queries/shooters/create_shooter.sql", id, name, default_class_id)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Shooter, "queries/shooters/get_shooter.sql", id)
                .fetch_one(&mut *txn)
                .await?
        }
        ShooterOperation::Delete {
            id,
        } => {
            let shooter = sqlx::query_file_as!(Shooter, "queries/shooters/get_shooter.sql", id)
                .fetch_one(&mut *txn)
                .await?;
            sqlx::query_file!("queries/shooters/delete_shooter.sql", id).execute(&mut *txn).await?;

            shooter
        }
        ShooterOperation::SetDefaultClass {
            id,
            default_class_id,
        } => {
            sqlx::query_file!("queries/shooters/set_default_class.sql", id, default_class_id)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Shooter, "queries/shooters/get_shooter.sql", id)
                .fetch_one(&mut *txn)
                .await?
        }
        ShooterOperation::SetName {
            id,
            name,
        } => {
            sqlx::query_file!("queries/shooters/set_name.sql", id, name).execute(&mut *txn).await?;
            sqlx::query_file_as!(Shooter, "queries/shooters/get_shooter.sql", id)
                .fetch_one(&mut *txn)
                .await?
        }
    };

    txn.commit().await?;
    Ok(Json(result))
}

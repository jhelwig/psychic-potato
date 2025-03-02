use axum::{
    Json,
    Router,
    routing::{
        get,
        post,
    },
};
use chrono::NaiveDate;
use shared_types::{
    request::MatchOperation,
    response::Match,
};

use crate::app::{
    AppState,
    DbTransaction,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_matches))
        .route("/operation", post(handle_match_operation))
        .with_state(app_state)
}

pub async fn list_matches(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<Match>>, crate::error::AppError> {
    let matches = sqlx::query_file_as!(Match, "queries/matches/list_matches.sql")
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(matches))
}
pub async fn handle_match_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Json(operation): Json<MatchOperation>,
) -> Result<Json<Match>, crate::error::AppError> {
    let result = match operation {
        MatchOperation::Create {
            name,
            event_date,
        } => {
            let match_id = uuid::Uuid::new_v4();
            sqlx::query_file!("queries/matches/create_match.sql", match_id, name, event_date)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", match_id)
                .fetch_one(&mut *txn)
                .await?
        }
        MatchOperation::SetDate {
            id,
            event_date,
        } => {
            sqlx::query_file!("queries/matches/set_date.sql", id, event_date)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", id)
                .fetch_one(&mut *txn)
                .await?
        }
        MatchOperation::SetName {
            id,
            name,
        } => {
            sqlx::query_file!("queries/matches/set_name.sql", id, name).execute(&mut *txn).await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", id)
                .fetch_one(&mut *txn)
                .await?
        }
        MatchOperation::Delete {
            id,
        } => {
            let r#match = sqlx::query_file_as!(Match, "queries/matches/get_match.sql", id)
                .fetch_one(&mut *txn)
                .await?;
            sqlx::query_file!("queries/matches/delete_match.sql", id).execute(&mut *txn).await?;
            r#match
        }
    };

    txn.commit().await?;
    Ok(Json(result))
}

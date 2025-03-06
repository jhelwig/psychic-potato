use axum::{
    Json,
    Router,
    extract::Path,
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
    #[error("Match not found: {match_id}")]
    NotFound {
        match_id: Uuid,
    },
}

pub fn league_match_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_league_matches))
        .route("/{match_id}", get(get_league_match))
        .route("/operation", post(handle_league_match_operation))
        .nest("/export", crate::app::export::router(app_state.clone()))
        .with_state(app_state)
}

async fn list_league_matches(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
) -> Result<Json<Vec<Match>>, AppError> {
    let result = sqlx::query_file_as!(Match, "queries/matches/list_matches.sql", league_id)
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(result))
}

async fn get_league_match(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((league_id, match_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Match>, AppError> {
    let result = sqlx::query_file_as!(Match, "queries/matches/get_match.sql", league_id, match_id)
        .fetch_optional(&mut *txn)
        .await?;
    let Some(match_object) = result else {
        return Err(MatchError::NotFound {
            match_id,
        }
        .into());
    };

    Ok(Json(match_object))
}
async fn handle_league_match_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
    Json(operation): Json<MatchOperation>,
) -> Result<Json<Match>, AppError> {
    let result = match operation {
        MatchOperation::Create {
            name,
            event_date,
        } => {
            let id = Uuid::new_v4();
            sqlx::query_file!("queries/matches/create_match.sql", id, league_id, name, event_date)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", league_id, id)
                .fetch_one(&mut *txn)
                .await?
        }
        MatchOperation::Delete {
            id,
        } => {
            let match_object =
                sqlx::query_file_as!(Match, "queries/matches/get_match.sql", league_id, id)
                    .fetch_one(&mut *txn)
                    .await?;
            sqlx::query_file!("queries/matches/delete_match.sql", id).execute(&mut *txn).await?;
            match_object
        }
        MatchOperation::SetDate {
            id,
            event_date,
        } => {
            sqlx::query_file!("queries/matches/set_date.sql", league_id, id, event_date)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", league_id, id)
                .fetch_one(&mut *txn)
                .await?
        }
        MatchOperation::SetName {
            id,
            name,
        } => {
            sqlx::query_file!("queries/matches/set_name.sql", league_id, id, name)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(Match, "queries/matches/get_match.sql", league_id, id)
                .fetch_one(&mut *txn)
                .await?
        }
    };

    txn.commit().await?;
    Ok(Json(result))
}

use axum::{
    Json,
    Router,
    extract::Path,
    routing::{
        get,
        post,
    },
};
use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use shared_types::{
    request::LeagueOperation,
    response::League,
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
pub enum LeagueError {
    #[error("League not found: {league_id}")]
    NotFound {
        league_id: Uuid,
    },
}

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_leagues))
        .nest("/{id}", single_league_router(app_state.clone()))
        .route("/operation", post(handle_league_operation))
        .with_state(app_state)
}

fn single_league_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_league))
        .nest("/match", crate::app::matches::league_match_router(app_state))
}

pub async fn list_leagues(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<League>>, AppError> {
    let leagues = sqlx::query_file_as!(League, "queries/leagues/list_leagues.sql")
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(leagues))
}

pub async fn get_league(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(id): Path<Uuid>,
) -> Result<Json<League>, AppError> {
    let league = sqlx::query_file_as!(League, "queries/leagues/get_league.sql", id)
        .fetch_optional(&mut *txn)
        .await?;

    let Some(league) = league else {
        return Err(LeagueError::NotFound {
            league_id: id,
        }
        .into());
    };

    Ok(Json(league))
}

pub async fn handle_league_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Json(operation): Json<LeagueOperation>,
) -> Result<Json<League>, AppError> {
    let result = match operation {
        LeagueOperation::Create {
            league_name,
        } => {
            let league_id = uuid::Uuid::new_v4();
            sqlx::query_file!("queries/leagues/create_league.sql", league_id, league_name)
                .execute(&mut *txn)
                .await?;
            sqlx::query_file_as!(League, "queries/leagues/get_league.sql", league_id)
                .fetch_one(&mut *txn)
                .await?
        }
        LeagueOperation::Delete {
            id,
        } => {
            let maybe_league = sqlx::query_file_as!(League, "queries/leagues/get_league.sql", id)
                .fetch_optional(&mut *txn)
                .await?;
            let Some(league) = maybe_league else {
                return Err(LeagueError::NotFound {
                    league_id: id,
                }
                .into());
            };
            sqlx::query_file!("queries/leagues/delete_league.sql", id).execute(&mut *txn).await?;

            league
        }
        LeagueOperation::SetDescription {
            id: _id,
            description: _description,
        } => todo!(),
        LeagueOperation::SetEndDate {
            id: _id,
            end_date: _end_date,
        } => todo!(),
        LeagueOperation::SetName {
            id,
            league_name,
        } => {
            sqlx::query_file!("queries/leagues/set_name.sql", id, league_name)
                .execute(&mut *txn)
                .await?;
            let maybe_league = sqlx::query_file_as!(League, "queries/leagues/get_league.sql", id)
                .fetch_optional(&mut *txn)
                .await?;

            let Some(league) = maybe_league else {
                return Err(LeagueError::NotFound {
                    league_id: id,
                }
                .into());
            };

            league
        }
        LeagueOperation::SetStartDate {
            id: _id,
            start_date: _start_date,
        } => todo!(),
    };

    txn.commit().await?;
    Ok(Json(result))
}

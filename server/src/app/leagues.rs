use anyhow::anyhow;
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
    Utc,
};
use log::info;
use shared_types::{
    request::LeagueOperation,
    response::{
        League,
        Match,
    },
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
    Router::new().route("/", get(get_league)).nest("/match", league_match_router(app_state))
}

fn league_match_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_league_matches))
        .route("/{match_id}", get(get_league_match))
        .route("/operation", post(handle_league_match_operation))
        .with_state(app_state)
}

async fn list_league_matches(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
) -> Result<Json<Vec<Match>>, AppError> {
    info!("Fetching matches for league: {}", league_id);

    Ok(Json(Vec::new()))
}
async fn get_league_match(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
    Path(match_id): Path<Uuid>,
) -> Result<Json<Match>, AppError> {
    info!("Fetching match: {} for league: {}", match_id, league_id);

    Err(anyhow!("Not implemented yet: Fetching match: {} for league: {}", match_id, league_id)
        .into())
}
async fn handle_league_match_operation(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path(league_id): Path<Uuid>,
) -> Result<Json<Match>, AppError> {
    info!("Handling league match operation for league: {}", league_id);

    Err(anyhow!("Not implemented yet: Handling league match operation for league: {}", league_id)
        .into())
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
    };

    txn.commit().await?;
    Ok(Json(result))
}

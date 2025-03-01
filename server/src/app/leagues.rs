use axum::{
    routing::{
        get,
        post,
    },
    Json,
    Router,
};
use chrono::{
    DateTime,
    Utc,
};
use shared_types::{
    request::LeagueOperation,
    response::League,
};
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        DbTransaction,
    },
    error::AppError,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_leagues))
        .route("/operation", post(handle_league_operation))
        .with_state(app_state)
}

pub async fn list_leagues(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<League>>, AppError> {
    let leagues = sqlx::query_file_as!(League, "queries/leagues/list_leagues.sql")
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(leagues))
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
        } => todo!(),
        LeagueOperation::SetName {
            id,
            league_name,
        } => todo!(),
    };

    txn.commit().await?;
    Ok(Json(result))
}

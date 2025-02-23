use axum::{
    routing::get,
    Json,
    Router,
};
use chrono::NaiveDate;
use shared_types::response::Match;

use crate::app::{
    AppState,
    DbTransaction,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(list_matches)).with_state(app_state)
}

pub async fn list_matches(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<Match>>, crate::error::AppError> {
    let matches = sqlx::query_file_as!(Match, "queries/matches/list_matches.sql",)
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(matches))
}

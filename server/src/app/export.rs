use axum::{
    routing::get,
    Json,
    Router,
};
use shared_types::response::ShotMarkerExport;

use crate::app::{
    AppState,
    DbTransaction,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(list_exports)).with_state(app_state)
}

pub async fn list_exports(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<ShotMarkerExport>>, crate::error::AppError> {
    let exports = sqlx::query_file_as!(ShotMarkerExport, "queries/export/list_exports.sql",)
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(exports))
}

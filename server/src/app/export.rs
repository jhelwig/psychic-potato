use anyhow::anyhow;
use axum::{
    Json,
    Router,
    extract::{
        DefaultBodyLimit,
        Multipart,
    },
    routing::get,
};
use shared_types::response::ShotMarkerExport;
use tower_http::limit::RequestBodyLimitLayer;

use crate::app::{
    AppState,
    DbTransaction,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(list_exports))
    .layer(DefaultBodyLimit::disable())
    .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10)) // 10MB)
    .with_state(app_state)
}

pub async fn list_exports(
    DbTransaction(mut txn): DbTransaction<'_>,
) -> Result<Json<Vec<ShotMarkerExport>>, crate::error::AppError> {
    let exports = sqlx::query_file_as!(ShotMarkerExport, "queries/export/list_exports.sql",)
        .fetch_all(&mut *txn)
        .await?;

    Ok(Json(exports))
}

pub async fn upload_export(
    // DbTransaction(mut txn): DbTransaction<'_>,
    mut multipart: Multipart,
) -> Result<(), crate::error::AppError> {
    while let Some(field) = multipart.next_field().await? {
        let _name = field.name().ok_or_else(|| anyhow!("Multipart missing name"))?.to_string();
        let _file_name =
            field.file_name().ok_or_else(|| anyhow!("Multipart missing file name"))?.to_string();
        let _content_type = field
            .content_type()
            .ok_or_else(|| anyhow!("Multipart missing content type"))?
            .to_string();
        let data = field.bytes().await?.to_vec();
        let _export_data = String::from_utf8(data)?;

        // let (_rest, export) = shotmarker_csv_parser::parser::export_parser(&export_data)?;
    }

    Ok(())
}

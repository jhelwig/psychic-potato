use axum::{
    Json,
    Router,
    extract::{
        DefaultBodyLimit,
        Path,
    },
    routing::{
        get,
        post,
    },
};
use chrono::Utc;
use log::info;
use shared_types::{
    request::SmCsvExportUpload,
    response::ShotMarkerExport,
};
use tower_http::limit::RequestBodyLimitLayer;
use uuid::Uuid;

use crate::app::{
    AppState,
    DbTransaction,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(list_exports))
    .route("/upload", post(upload_export))
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
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((league_id, match_id)): Path<(Uuid, Uuid)>,
    Json(upload): Json<SmCsvExportUpload>,
) -> Result<(), crate::error::AppError> {
    let file_name = upload.filename;
    let export_data = upload.content;
    info!(
        "Received export for league {league_id} and match {match_id}: {file_name} ({})",
        export_data.len()
    );

    let (_rest, export) = shotmarker_csv_parser::parser::export_parser(&export_data)?;

    let export_id = Uuid::new_v4();
    let string_count = i32::try_from(export.string_count)?;
    sqlx::query_file!(
        "queries/export/create_export.sql",
        export_id,
        file_name,
        export.generated_date,
        string_count,
        export.string_date,
        match_id,
    )
    .execute(&mut *txn)
    .await?;

    let export_file_id = Uuid::new_v4();
    let uploaded_at = Utc::now();
    sqlx::query_file!(
        "queries/export_files/create_export_file.sql",
        export_file_id,
        file_name,
        export_data,
        uploaded_at,
        export_id,
    )
    .execute(&mut *txn)
    .await?;

    for shot_string in &export.strings {
        let shot_string_id = Uuid::new_v4();
        let name = shot_string.name.clone();
        let target = shot_string.target.clone();
        let distance = shot_string.distance.clone();
        let score = shot_string.score.clone();
        sqlx::query_file!(
            "queries/shot_strings/create_shot_string.sql",
            shot_string_id,
            shot_string.date,
            name,
            target,
            distance,
            score,
            export_id,
        )
        .execute(&mut *txn)
        .await?;

        for shot in &shot_string.shots {
            let shot_id = Uuid::new_v4();
            let string_shot_id = shot.id.clone();
            let tags = shot.tags.clone();
            let serialized_score = serde_json::to_value(shot.score)?;
            let serialized_position = serde_json::to_value(shot.position)?;
            let serialized_velocity = serde_json::to_value(shot.velocity)?;
            sqlx::query_file!(
                "queries/shots/create_shot.sql",
                shot_id,
                shot.time,
                string_shot_id,
                tags,
                serialized_score,
                serialized_position,
                serialized_velocity,
                shot.yaw,
                shot.pitch,
                shot.quality,
                shot_string_id,
            )
            .execute(&mut *txn)
            .await?;
        }
    }

    txn.commit().await?;
    Ok(())
}

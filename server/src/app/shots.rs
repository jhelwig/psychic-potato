use axum::{
    Json,
    Router,
    extract::Path,
    routing::get,
};
use chrono::NaiveTime;
use shared_types::response::{
    ShotMarkerShot,
    ShotPosition,
    ShotScore,
    ShotVelocity,
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
    Router::new().route("/", get(index)).with_state(app_state)
}

#[derive(sqlx::FromRow)]
struct SqlxShotMarkerShot {
    id:             Uuid,
    shot_time:      NaiveTime,
    shot_id:        String,
    tags:           String,
    score:          sqlx::types::Json<ShotScore>,
    position:       sqlx::types::Json<ShotPosition>,
    velocity:       sqlx::types::Json<ShotVelocity>,
    yaw:            f64,
    pitch:          f64,
    quality:        Option<String>,
    shot_string_id: Uuid,
}

impl From<SqlxShotMarkerShot> for ShotMarkerShot {
    fn from(value: SqlxShotMarkerShot) -> Self {
        ShotMarkerShot {
            id:             value.id,
            shot_time:      value.shot_time,
            shot_id:        value.shot_id,
            tags:           value.tags,
            score:          value.score.0,
            position:       value.position.0,
            velocity:       value.velocity.0,
            yaw:            value.yaw,
            pitch:          value.pitch,
            quality:        value.quality,
            shot_string_id: value.shot_string_id,
        }
    }
}

async fn index(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((league_id, match_id, shot_string_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<Vec<ShotMarkerShot>>, AppError> {
    let shots = sqlx::query_file_as!(
        SqlxShotMarkerShot,
        "queries/shots/list_shots_for_string.sql",
        league_id,
        match_id,
        shot_string_id,
    )
    .fetch_all(&mut *txn)
    .await?;

    Ok(Json(shots.into_iter().map(Into::into).collect()))
}

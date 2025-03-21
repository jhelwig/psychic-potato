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
use shared_types::response::ShotMarkerShotString;
use shotmarker_csv_parser::string::StringScore;
use uuid::Uuid;

use crate::{
    app::{
        AppState,
        DbTransaction,
    },
    error::{
        AppError,
        HttpResponse,
    },
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .nest("/{string_id}", single_string_router(app_state.clone()))
        .route("/operation", post(handle_string_operation))
        .with_state(app_state)
}

fn single_string_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_string))
        .nest("/shot", crate::app::shots::router(app_state.clone()))
        .with_state(app_state)
}

#[derive(sqlx::FromRow)]
struct SqlxShotMarkerShotString {
    id:          Uuid,
    string_date: NaiveDate,
    string_name: String,
    target:      String,
    distance:    String,
    score:       sqlx::types::Json<StringScore>,
    export_id:   Uuid,
    shooter_id:  Option<Uuid>,
    class_id:    Option<Uuid>,
}

impl From<SqlxShotMarkerShotString> for ShotMarkerShotString {
    fn from(value: SqlxShotMarkerShotString) -> Self {
        ShotMarkerShotString {
            id:          value.id,
            string_date: value.string_date,
            string_name: value.string_name,
            target:      value.target,
            distance:    value.distance,
            score:       value.score.0,
            export_id:   value.export_id,
            shooter_id:  value.shooter_id,
            class_id:    value.class_id,
        }
    }
}

async fn index(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((_league_id, match_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ShotMarkerShotString>>, AppError> {
    let strings = sqlx::query_file_as!(
        SqlxShotMarkerShotString,
        "queries/shot_strings/list_shot_strings_for_match.sql",
        match_id,
    )
    .fetch_all(&mut *txn)
    .await?;

    Ok(Json(strings.into_iter().map(Into::into).collect()))
}

async fn get_string(
    DbTransaction(mut txn): DbTransaction<'_>,
    Path((_league_id, match_id, string_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<ShotMarkerShotString>, AppError> {
    let string = sqlx::query_file_as!(
        SqlxShotMarkerShotString,
        "queries/shot_strings/get_shot_string.sql",
        match_id,
        string_id,
    )
    .fetch_optional(&mut *txn)
    .await?;

    let Some(shot_string) = string else {
        return Err(HttpResponse::NotFound {
            message: "Shot string not found.".into(),
        }
        .into());
    };

    Ok(Json(shot_string.into()))
}

async fn handle_string_operation() -> Result<Json<ShotMarkerShotString>, AppError> {
    //
    todo!()
}

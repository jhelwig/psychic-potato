use chrono::{
    DateTime,
    NaiveDate,
    NaiveTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use shotmarker_csv_parser::string::shot::{
    ShotPosition,
    ShotScore,
    ShotVelocity,
};
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct League {
    pub id:         Uuid,
    pub name:       String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Match {
    pub id:         Uuid,
    pub name:       String,
    pub event_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotMarkerExport {
    pub id:             Uuid,
    pub file_name:      String,
    pub generated_date: NaiveDate,
    pub string_count:   i32,
    pub string_date:    NaiveDate,
    pub match_id:       Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerShotString {
    pub id:          Uuid,
    pub string_date: NaiveDate,
    pub string_name: String,
    pub target:      String,
    pub distance:    String,
    pub score:       String,
    pub export_id:   Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerShot {
    pub id:        Uuid,
    pub shot_time: NaiveTime,
    pub shot_id:   String,
    pub tags:      String,
    pub score:     ShotScore,
    pub position:  ShotPosition,
    pub velocity:  ShotVelocity,
    pub yaw:       f64,
    pub pitch:     f64,
    pub quality:   Option<String>,
}

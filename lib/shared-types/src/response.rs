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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct League {
    pub id:          Uuid,
    pub name:        String,
    pub description: Option<String>,
    pub created_at:  DateTime<Utc>,
    pub start_date:  Option<NaiveDate>,
    pub end_date:    Option<NaiveDate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Match {
    pub id:         Uuid,
    pub name:       String,
    pub event_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShotMarkerExport {
    pub id:             Uuid,
    pub file_name:      String,
    pub generated_date: NaiveDate,
    pub string_count:   i32,
    pub string_date:    NaiveDate,
    pub match_id:       Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShotMarkerShotString {
    pub id:          Uuid,
    pub string_date: NaiveDate,
    pub string_name: String,
    pub target:      String,
    pub distance:    String,
    pub score:       String,
    pub export_id:   Uuid,
    pub shooter_id:  Option<Uuid>,
    pub class_id:    Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotMarkerShot {
    pub id:             Uuid,
    pub shot_time:      NaiveTime,
    pub shot_id:        String,
    pub tags:           String,
    pub score:          ShotScore,
    pub position:       ShotPosition,
    pub velocity:       ShotVelocity,
    pub yaw:            f64,
    pub pitch:          f64,
    pub quality:        Option<String>,
    pub shot_string_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Class {
    pub id:          Uuid,
    pub name:        String,
    pub description: Option<String>,
    pub league_id:   Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shooter {
    pub id:               Uuid,
    pub name:             String,
    pub default_class_id: Option<Uuid>,
}

use crate::string::shot::ShotMarkerShot;
use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};

pub mod shot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerStringMetrics {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerShotString {
    pub date:     NaiveDate,
    pub name:     String,
    pub target:   String,
    pub distance: String,
    pub score:    String,
    pub shots:    Vec<ShotMarkerShot>,
    pub metrics:  Option<ShotMarkerStringMetrics>,
}

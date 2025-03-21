use std::cmp::Ordering;

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
pub struct StringScore {
    pub points:  u32,
    pub x_count: u32,
}

impl std::fmt::Display for StringScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}X", self.points, self.x_count)
    }
}

impl Ord for StringScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points.cmp(&other.points).then(self.x_count.cmp(&other.x_count))
    }
}

impl PartialOrd for StringScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for StringScore {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.x_count == other.x_count
    }
}

impl Eq for StringScore {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerShotString {
    pub date:     NaiveDate,
    pub name:     String,
    pub target:   String,
    pub distance: String,
    pub score:    StringScore,
    pub shots:    Vec<ShotMarkerShot>,
    pub metrics:  Option<ShotMarkerStringMetrics>,
}

use crate::string::ShotMarkerShotString;
use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};

pub mod error;
pub mod parser;
pub mod string;
pub mod units;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotMarkerExport {
    pub generated_date: NaiveDate,
    pub string_count:   usize,
    pub string_date:    NaiveDate,
    pub strings:        Vec<ShotMarkerShotString>,
}

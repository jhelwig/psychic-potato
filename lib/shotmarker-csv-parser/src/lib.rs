use crate::string::ShotMarkerShotString;
use chrono::NaiveDate;

pub mod error;
pub mod parser;
pub mod string;
pub mod units;

#[derive(Debug, Clone)]
pub struct ShotMarkerExport {
    pub generated_date: NaiveDate,
    pub string_count:   usize,
    pub string_date:    NaiveDate,
    pub strings:        Vec<ShotMarkerShotString>,
}

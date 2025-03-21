use std::cmp::Ordering;

use crate::units::{
    Inches,
    Mil,
    Millimeters,
    Moa,
};
use chrono::NaiveTime;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotVelocity {
    pub ms:  f64,
    pub fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotMarkerShot {
    pub time:     NaiveTime,
    pub id:       String,
    pub tags:     String,
    pub score:    ShotScore,
    pub position: ShotPosition,
    pub velocity: ShotVelocity,
    pub yaw:      f64,
    pub pitch:    f64,
    pub quality:  Option<f64>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotXYmm {
    pub x: Millimeters,
    pub y: Millimeters,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotXYinch {
    pub x: Inches,
    pub y: Inches,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotXYmoa {
    pub x: Moa,
    pub y: Moa,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotXYmil {
    pub x: Mil,
    pub y: Mil,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShotPosition {
    pub mm:   ShotXYmm,
    pub inch: ShotXYinch,
    pub moa:  ShotXYmoa,
    pub mil:  ShotXYmil,
}

#[remain::sorted]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ShotScore {
    None,
    Numeric(u8),
    X,
}

impl std::fmt::Display for ShotScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Numeric(n) => write!(f, "{}", n),
            Self::X => write!(f, "X"),
        }
    }
}

impl Ord for ShotScore {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ShotScore::None, ShotScore::None) | (ShotScore::X, ShotScore::X) => Ordering::Equal,
            (ShotScore::Numeric(a), ShotScore::Numeric(b)) => a.cmp(b),
            (ShotScore::None, ShotScore::Numeric(_))
            | (ShotScore::None, ShotScore::X)
            | (ShotScore::Numeric(_), ShotScore::X) => Ordering::Less,
            (ShotScore::Numeric(_), ShotScore::None)
            | (ShotScore::X, ShotScore::None)
            | (ShotScore::X, ShotScore::Numeric(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for ShotScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for ShotScore {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}

impl Eq for ShotScore {}

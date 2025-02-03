use crate::units::{
    Inches,
    Mil,
    Millimeters,
    Moa,
};
use chrono::NaiveTime;

#[derive(Debug, Copy, Clone)]
pub struct ShotVelocity {
    pub ms:  f64,
    pub fps: u32,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct ShotXYmm {
    pub x: Millimeters,
    pub y: Millimeters,
}

#[derive(Debug, Copy, Clone)]
pub struct ShotXYinch {
    pub x: Inches,
    pub y: Inches,
}

#[derive(Debug, Copy, Clone)]
pub struct ShotXYmoa {
    pub x: Moa,
    pub y: Moa,
}

#[derive(Debug, Copy, Clone)]
pub struct ShotXYmil {
    pub x: Mil,
    pub y: Mil,
}

#[derive(Debug, Copy, Clone)]
pub struct ShotPosition {
    pub mm:   ShotXYmm,
    pub inch: ShotXYinch,
    pub moa:  ShotXYmoa,
    pub mil:  ShotXYmil,
}

#[remain::sorted]
#[derive(Debug, Copy, Clone)]
pub enum ShotScore {
    None,
    Numeric(u8),
    X,
}

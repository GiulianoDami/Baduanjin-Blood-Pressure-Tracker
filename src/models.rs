use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodPressureRecord {
    pub id: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub systolic: u32,
    pub diastolic: u32,
    pub heart_rate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseSession {
    pub id: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub duration_minutes: u32,
    pub session_notes: Option<String>,
}
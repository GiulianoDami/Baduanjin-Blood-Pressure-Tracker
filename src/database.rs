use chrono::prelude::*;
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BloodPressure {
    pub systolic: i32,
    pub diastolic: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseSession {
    pub duration_minutes: i32,
    pub timestamp: DateTime<Utc>,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("blood_pressure_tracker.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blood_pressure (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            systolic INTEGER NOT NULL,
            diastolic INTEGER NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exercise_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            duration_minutes INTEGER NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn save_blood_pressure(conn: &Connection, bp: &BloodPressure) -> Result<()> {
    conn.execute(
        "INSERT INTO blood_pressure (systolic, diastolic, timestamp) VALUES (?1, ?2, ?3)",
        params![bp.systolic, bp.diastolic, bp.timestamp.to_rfc3339()],
    )?;
    Ok(())
}

pub fn save_exercise_session(conn: &Connection, session: &ExerciseSession) -> Result<()> {
    conn.execute(
        "INSERT INTO exercise_sessions (duration_minutes, timestamp) VALUES (?1, ?2)",
        params![session.duration_minutes, session.timestamp.to_rfc3339()],
    )?;
    Ok(())
}
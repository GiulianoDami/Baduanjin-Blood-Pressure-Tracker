use crate::database::{get_all_blood_pressure_readings, get_exercise_log};
use chrono::{Datelike, NaiveDate};
use plotters::prelude::*;
use rusqlite::Result as SqliteResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BpReading {
    pub date: String,
    pub systolic: i32,
    pub diastolic: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseEntry {
    pub date: String,
    pub duration_minutes: i32,
}

pub fn generate_bp_trend_chart(output_path: &str) -> SqliteResult<()> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Blood Pressure Trend Over Time", TextStyle::from(("sans-serif", 20)))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()..NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            80i32..180i32,
        )?;
    
    chart.configure_mesh().draw()?;
    
    let readings = get_all_blood_pressure_readings()?;
    
    if readings.is_empty() {
        return Ok(());
    }
    
    let points: Vec<(NaiveDate, i32)> = readings
        .iter()
        .filter_map(|r| {
            let date = NaiveDate::parse_from_str(&r.date, "%Y-%m-%d").ok()?;
            Some((date, r.systolic))
        })
        .collect();
    
    chart
        .draw_series(LineSeries::new(
            points.iter().map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("Systolic BP")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    chart.configure_series_labels().draw()?;
    
    Ok(())
}

pub fn generate_exercise_frequency_chart(output_path: &str) -> SqliteResult<()> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Exercise Frequency Over Time", TextStyle::from(("sans-serif", 20)))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()..NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            0i32..120i32,
        )?;
    
    chart.configure_mesh().draw()?;
    
    let entries = get_exercise_log()?;
    
    if entries.is_empty() {
        return Ok(());
    }
    
    let points: Vec<(NaiveDate, i32)> = entries
        .iter()
        .filter_map(|e| {
            let date = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok()?;
            Some((date, e.duration_minutes))
        })
        .collect();
    
    chart
        .draw_series(LineSeries::new(
            points.iter().map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("Exercise Duration")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    chart.configure_series_labels().draw()?;
    
    Ok(())
}
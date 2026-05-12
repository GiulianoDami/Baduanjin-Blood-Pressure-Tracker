use clap::{ArgMatches, Command};
use rusqlite::Connection;
use std::io::{self, Write};

pub fn handle_command(matches: &ArgMatches, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("add", submatches)) => {
            if let (Some(systolic), Some(diastolic), Some(date)) = (
                submatches.value_of("SYSTOLIC").map(|s| s.parse::<i32>().unwrap()),
                submatches.value_of("DIASTOLIC").map(|s| s.parse::<i32>().unwrap()),
                submatches.value_of("DATE"),
            ) {
                // Add blood pressure reading to database
                let mut stmt = conn.prepare("INSERT INTO readings (systolic, diastolic, date) VALUES (?, ?, ?)")?;
                stmt.execute([systolic, diastolic, date.to_string()])?;
                println!("Blood pressure reading added successfully!");
            } else {
                eprintln!("Error: Please provide valid systolic, diastolic, and date values.");
            }
        }
        Some(("view", _)) => {
            // View all blood pressure readings
            let mut stmt = conn.prepare("SELECT * FROM readings ORDER BY date DESC")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get(1)?, row.get(2)?, row.get(3)?))
            })?;

            println!("{:<10} {:<10} {:<12}", "Systolic", "Diastolic", "Date");
            println!("----------------------------------------");
            for row in rows {
                let (systolic, diastolic, date) = row?;
                println!("{:<10} {:<10} {:<12}", systolic, diastolic, date);
            }
        }
        Some(("export", _)) => {
            export_csv(conn)?;
        }
        Some(("help", _)) => {
            print_help();
        }
        _ => {
            print_help();
        }
    }
    Ok(())
}

pub fn print_help() {
    println!("Baduanjin Blood Pressure Tracker");
    println!("Usage: baduanjin [COMMAND]");
    println!();
    println!("Commands:");
    println!("  add         Add a new blood pressure reading");
    println!("  view        View all blood pressure readings");
    println!("  export      Export data to CSV");
    println!("  help        Show this help message");
    println!();
    println!("Examples:");
    println!("  baduanjin add 120 80 2023-05-01");
    println!("  baduanjin view");
    println!("  baduanjin export");
}

pub fn export_csv(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM readings ORDER BY date DESC")?;
    let mut rows = stmt.query([])?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "systolic,diastolic,date")?;
    
    while let Some(row) = rows.next()? {
        let systolic: i32 = row.get(1)?;
        let diastolic: i32 = row.get(2)?;
        let date: String = row.get(3)?;
        writeln!(handle, "{},{},{}", systolic, diastolic, date)?;
    }

    println!("Data exported to 'readings.csv'");
    Ok(())
}
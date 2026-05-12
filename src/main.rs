use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Baduanjin Blood Pressure Tracker")]
#[command(version = "1.0")]
#[command(about = "Track blood pressure improvements through Baduanjin practice", long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value = "~/.baduanjin_tracker.db")]
    database_path: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new blood pressure reading
    AddReading {
        #[arg(short, long)]
        systolic: u16,
        
        #[arg(short, long)]
        diastolic: u16,
        
        #[arg(short, long, default_value = "now")]
        timestamp: String,
    },
    
    /// Record Baduanjin practice session
    AddPractice {
        #[arg(short, long)]
        duration_minutes: u32,
        
        #[arg(short, long, default_value = "now")]
        timestamp: String,
    },
    
    /// View tracking statistics
    Stats,
    
    /// Generate progress visualization
    Plot,
    
    /// Set exercise goal
    SetGoal {
        #[arg(short, long)]
        weekly_sessions: u32,
    },
}

fn main() {
    let args = CliArgs::parse();
    
    match &args.command {
        Commands::AddReading { systolic, diastolic, timestamp } => {
            println!("Adding reading: {} / {} at {}", systolic, diastolic, timestamp);
        }
        Commands::AddPractice { duration_minutes, timestamp } => {
            println!("Adding practice: {} minutes at {}", duration_minutes, timestamp);
        }
        Commands::Stats => {
            println!("Showing statistics...");
        }
        Commands::Plot => {
            println!("Generating plot...");
        }
        Commands::SetGoal { weekly_sessions } => {
            println!("Setting goal: {} weekly sessions", weekly_sessions);
        }
    }
}
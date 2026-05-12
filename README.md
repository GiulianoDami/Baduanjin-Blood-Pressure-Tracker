PROJECT_NAME: Baduanjin Blood Pressure Tracker

# Baduanjin Blood Pressure Tracker

A Rust-based application that helps users track and monitor their blood pressure improvements through the practice of Baduanjin (Eight-Section Brocade), an ancient Chinese exercise routine.

## Description

This project addresses the growing need for accessible, evidence-based health monitoring tools that can help individuals track the natural blood pressure-lowering benefits of traditional Chinese exercises. Inspired by recent clinical studies showing that Baduanjin can significantly reduce blood pressure in adults with stage 1 hypertension, this application provides a simple yet effective way to:

- Track blood pressure readings over time
- Monitor Baduanjin practice frequency and duration
- Visualize improvement patterns
- Set personalized goals for exercise consistency
- Provide gentle reminders for daily practice

The application is built with Rust for performance, reliability, and cross-platform compatibility, making it perfect for health-conscious users who want to scientifically track their wellness journey.

## Installation

### Prerequisites
- Rust 1.60 or later installed
- Cargo package manager (comes with Rust)

### Installation Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/baduanjin-tracker.git
cd baduanjin-tracker

# Build the project
cargo build --release

# Run the application
cargo run
```

Alternatively, you can install directly from crates.io:
```bash
cargo install baduanjin-tracker
```

## Usage

### Basic Commands

```bash
# Start the interactive tracker
cargo run

# View help information
cargo run --help

# Export data to CSV
cargo run --export-csv

# View statistics
cargo run --stats
```

### Features

1. **Blood Pressure Logging**: Record systolic/diastolic readings with timestamps
2. **Exercise Tracking**: Log Baduanjin practice sessions with duration and intensity
3. **Progress Visualization**: Generate charts showing blood pressure trends over time
4. **Goal Setting**: Set personal targets for both blood pressure reduction and exercise frequency
5. **Reminder System**: Configure daily reminders for Baduanjin practice
6. **Data Export**: Export all tracked data for sharing with healthcare providers

### Example Workflow

```bash
# Initialize your health profile
cargo run --init

# Log a blood pressure reading
cargo run --bp 120/80

# Log a Baduanjin session
cargo run --exercise 15m

# View your progress
cargo run --progress

# Export data
cargo run --export
```

### Data Storage

All data is stored locally in a SQLite database file (`health_data.db`) in the application directory. The database schema includes tables for:
- Blood pressure readings
- Exercise sessions
- User profiles
- Goal settings

## Technical Details

This Rust application leverages:
- `clap` for command-line argument parsing
- `rusqlite` for local database management
- `chrono` for timestamp handling
- `plotters` for data visualization
- `serde` for data serialization

The application follows Rust best practices with proper error handling, memory safety, and efficient resource management.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details.

## Acknowledgments

Inspired by recent clinical research on Baduanjin's effectiveness in lowering blood pressure naturally, this project aims to make traditional wellness practices more accessible through modern technology.
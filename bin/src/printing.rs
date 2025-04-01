use crate::models::Stats;
use chrono::Utc;
use num_format::{Locale, ToFormattedString};
use std::sync::{Arc, Mutex};
use tokio::io;

// Function to print statistics about the log entries
pub async fn print_stats(stats: Arc<Mutex<Stats>>) -> io::Result<()> {
    loop {
        // Sleep for 60 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Print the statistics
        let stats = stats.lock().unwrap();

        let now = Utc::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

        // Print the count of each log level in the past 60 seconds
        // clear the screen
        print!("\x1B[2J\x1B[1;1H");
        println!("Log Analysis Report (Last Updated: {})", now);
        println!("═══════════════════════════════════════════════════════════");
        println!("Runtime Stats:");
        println!(
            "- Total Entries: {}",
            stats.raw.total_count.to_formatted_string(&Locale::en)
        );
        println!();
        println!("Pattern Analysis:");
        println!(
            "- Debug: {} ({} entries)",
            format_percentage(stats.calculated.debug_pct),
            stats.raw.debug_count.to_formatted_string(&Locale::en)
        );
        println!(
            "- Info: {} ({} entries)",
            format_percentage(stats.calculated.info_pct),
            stats.raw.info_count.to_formatted_string(&Locale::en)
        );
        println!(
            "- Error: {} ({} entries)",
            format_percentage(stats.calculated.error_pct),
            stats.raw.error_count.to_formatted_string(&Locale::en)
        );
        println!("═══════════════════════════════════════════════════════════");
        println!("Press Ctrl+C to exit.");
    }
}

fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

use crate::models::Stats;
use parsing::LogEntry;
use std::sync::{Arc, Mutex};
use tokio::io;

// Function to analyze queue and calculate statistics
pub async fn analyze_queue(
    queue: Arc<Mutex<Vec<LogEntry>>>,
    stats: Arc<Mutex<Stats>>,
) -> io::Result<()> {
    // Flush from the queue
    loop {
        // Sleep for 1 second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Lock the queue and analyze the entries
        let mut queue = queue.lock().unwrap();
        let mut stats = stats.lock().unwrap();

        // Update raw stats
        for entry in queue.iter() {
            stats.raw.total_count += 1;
            match entry.level {
                parsing::LogLevel::Error => stats.raw.error_count += 1,
                parsing::LogLevel::Info => stats.raw.info_count += 1,
                parsing::LogLevel::Debug => stats.raw.debug_count += 1,
            }
        }

        // Calculate remaining stats
        stats.calculated.error_pct = stats.raw.error_count as f64 / stats.raw.total_count as f64;
        stats.calculated.info_pct = stats.raw.info_count as f64 / stats.raw.total_count as f64;
        stats.calculated.debug_pct = stats.raw.debug_count as f64 / stats.raw.total_count as f64;

        // TODO: Fix
        stats.calculated.avg_entries_per_second =
            (stats.raw.total_count as f64 / 60.0).round() as usize;
        // TODO: Fix
        stats.calculated.peak_entries_per_second = stats.raw.total_count;
        // TODO: Fix
        stats.calculated.entries_last_second = stats.raw.total_count;

        // Clear the queue
        queue.clear();
    }
}

mod analysis;
mod models;
mod printing;
mod streaming;

use crate::models::Stats;
use chrono::Utc;
use num_format::{Locale, ToFormattedString};
use parsing::LogEntry;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let debug_flag: bool = args.contains(&"-debug".to_string());

    // Basic queue will just be number of errors in the past 60 seconds
    let queue: Arc<Mutex<Vec<LogEntry>>> = Arc::new(Mutex::new(Vec::new()));
    let stats: Arc<Mutex<Stats>> = Arc::new(Mutex::new(Stats::default()));

    // Spawn a new thread to process stdin
    let queue_clone = queue.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = process_stdin(debug_flag, queue_clone.clone()).await {
            eprintln!("Error processing stdin: {}", e);
        }
    });

    // Spawn a new thread to print statistics
    let stats_clone = stats.clone();
    let stats_handle = tokio::spawn(async move {
        if let Err(e) = print_stats(debug_flag, stats_clone.clone()).await {
            eprintln!("Error printing stats: {}", e);
        }
    });

    // Spawn a new thread to analyze the queue
    let analyze_clone = queue.clone();
    let stats_clone = stats.clone();
    let analyze_handle = tokio::spawn(async move {
        if let Err(e) = analyze_queue(debug_flag, analyze_clone.clone(), stats_clone.clone()).await
        {
            eprintln!("Error analyzing queue: {}", e);
        }
    });

    // Wait for the thread to finish
    handle.await?;
    stats_handle.await?;
    analyze_handle.await?;

    Ok(())
}

// Function to run on separate thread
async fn process_stdin(debug_flag: bool, queue: Arc<Mutex<Vec<LogEntry>>>) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        // Parse the line
        let log_entry: LogEntry = match line.try_into() {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error parsing log entry: {}", e);
                continue;
            }
        };
        if debug_flag {
            println!("{:?}", log_entry);
        }

        let mut queue = queue.lock().unwrap();
        queue.push(log_entry);
    }

    Ok(())
}

// Function to print statistics about the log entries
async fn print_stats(debug_flag: bool, stats: Arc<Mutex<Stats>>) -> io::Result<()> {
    loop {
        // Sleep for 60 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Print the statistics
        let stats = stats.lock().unwrap();

        let now = Utc::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

        fn format_percentage(value: f64) -> String {
            format!("{:.2}%", value * 100.0)
        }

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
        println!(
            "- Current Rate: {} entries/sec (Peak: {} entries/sec)",
            stats
                .calculated
                .avg_entries_per_second
                .to_formatted_string(&Locale::en),
            stats
                .calculated
                .peak_entries_per_second
                .to_formatted_string(&Locale::en)
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

// Function to analyze queue and calculate statistics
async fn analyze_queue(
    debug_flag: bool,
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

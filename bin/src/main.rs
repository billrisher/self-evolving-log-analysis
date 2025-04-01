mod analysis;
mod models;
mod printing;
mod streaming;

use crate::analysis::analyze_queue;
use crate::models::Stats;
use crate::printing::print_stats;
use crate::streaming::process_stdin;
use parsing::LogEntry;
use std::sync::{Arc, Mutex};
use tokio::io::{self};

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
            if debug_flag {
                eprintln!("Error processing stdin: {}", e);
            } else {
                eprintln!("Error processing stdin");
            }
        }
    });

    // Spawn a new thread to print statistics
    let stats_clone = stats.clone();
    let stats_handle = tokio::spawn(async move {
        if let Err(e) = print_stats(stats_clone.clone()).await {
            if debug_flag {
                eprintln!("Error printing stats: {}", e);
            } else {
                eprintln!("Error printing stats");
            }
        }
    });

    // Spawn a new thread to analyze the queue
    let analyze_clone = queue.clone();
    let stats_clone = stats.clone();
    let analyze_handle = tokio::spawn(async move {
        if let Err(e) = analyze_queue(analyze_clone.clone(), stats_clone.clone()).await {
            if debug_flag {
                eprintln!("Error analyzing queue: {}", e);
            } else {
                eprintln!("Error analyzing queue");
            }
        }
    });

    // Wait for the thread to finish
    handle.await?;
    stats_handle.await?;
    analyze_handle.await?;

    Ok(())
}

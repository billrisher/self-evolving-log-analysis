use parsing::LogEntry;
use std::sync::{Arc, Mutex};
use tokio::io;
use tokio::io::AsyncBufReadExt;

// Function to run on separate thread
pub async fn process_stdin(debug_flag: bool, queue: Arc<Mutex<Vec<LogEntry>>>) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        // Parse the line
        let log_entry: LogEntry = match line.try_into() {
            Ok(entry) => entry,
            Err(e) => {
                if debug_flag {
                    eprintln!("Error parsing log entry: {}", e);
                }
                continue;
            }
        };

        let mut queue = queue.lock().unwrap();
        queue.push(log_entry);
    }

    Ok(())
}

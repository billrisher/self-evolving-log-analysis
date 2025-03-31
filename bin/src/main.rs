use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line?;

        // Parse the line
        let log_entry: parsing::LogEntry = match line.clone().try_into() {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error parsing log entry: {}", e);
                continue;
            }
        };
        // Process each line
        println!("{:?}", log_entry);
    }

    Ok(())
}

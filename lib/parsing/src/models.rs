use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

// Lazily declare the REGEX pattern we're expecting log to fit
lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"\[(?P<timestamp>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)\] (?P<level>\w+) - IP:(?P<ip>\d+.\d+.\d+.\d+)\s*(?P<message>.*)"
    ).expect("Failed to compile regex");
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Failed to parse log entry")]
    ParseError(&'static str),
    #[error("Failed to parse timestamp")]
    TimestampError(#[from] chrono::ParseError),
    #[error("Failed to parse log level")]
    LogLevelError(#[from] std::str::Utf8Error),
    #[error("Unknown log level")]
    UnknownLogLevel,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LogLevel {
    Info,
    Error,
    Debug,
}

impl LogLevel {
    pub fn from_str(level: &str) -> Result<Self, ParsingError> {
        match level {
            "INFO" => Ok(LogLevel::Info),
            "ERROR" => Ok(LogLevel::Error),
            "DEBUG" => Ok(LogLevel::Debug),
            _ => Err(ParsingError::UnknownLogLevel),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: Option<String>,
}

impl TryFrom<String> for LogEntry {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Parse the log entry using the regex
        let captures = match REGEX.captures(&value) {
            Some(c) => c,
            None => return Err(ParsingError::ParseError("Failed to match regex")),
        };

        // Extract the fields from the captures

        // Extract the timestamp, parse it, and convert to UTC
        let timestamp = match captures.name("timestamp") {
            Some(t) => match DateTime::parse_from_rfc3339(t.as_str()) {
                Ok(t) => t.with_timezone(&Utc),
                Err(err) => return Err(ParsingError::TimestampError(err)),
            },
            None => return Err(ParsingError::ParseError("Failed to parse timestamp")),
        };

        // Extract the log level, and convert to LogLevel enum
        let level = match captures.name("level") {
            Some(l) => match LogLevel::from_str(l.as_str()) {
                Ok(l) => l,
                Err(err) => return Err(err),
            },
            None => return Err(ParsingError::ParseError("Failed to parse log level")),
        };

        let message = captures
            .name("message")
            .map(|m| m.as_str().to_string())
            .filter(|m| !m.is_empty());

        // Create a new LogEntry struct
        Ok(LogEntry {
            timestamp,
            level,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_complete_logs() {
        let debug_entry =
            "[2023-10-01T12:00:00Z] ERROR - IP:192.168.123.45 Error 500 - This is a debug message";

        let entry = LogEntry::try_from(debug_entry.to_string()).expect("Failed to parse log entry");
        assert_eq!(entry.timestamp.to_string(), "2023-10-01 12:00:00 UTC");
        assert_eq!(entry.level, LogLevel::Debug);
        assert_eq!(
            entry.message,
            Some("Error 500 - This is a debug message".to_string())
        );
        assert_eq!(entry.timestamp.to_string(), "2023-10-01 12:00:00 UTC");
    }

    #[test]
    fn parse_log_without_message() {
        let log_entry = "[2024-01-28T15:30:45Z] DEBUG - IP:192.168.234.12 ";

        let entry = LogEntry::try_from(log_entry.to_string()).expect("Failed to parse log entry");
        assert_eq!(entry.timestamp.to_string(), "2024-01-28 15:30:45 UTC");
        assert_eq!(entry.level, LogLevel::Debug);
        assert_eq!(entry.message, None);
    }

    #[test]
    fn parse_malformed_log_returns_error() {
        let log_entry =
            "[24=01-29T15:3:45Z] DEBG - IP:19:168.234.12 - This is a malformed log entry";

        let result = LogEntry::try_from(log_entry.to_string());
        assert!(result.is_err());
    }
}

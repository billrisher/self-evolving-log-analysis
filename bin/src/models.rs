#[derive(Debug, Default, Clone)]
pub struct Stats {
    /// Stats that do not require processing
    pub raw: RawStats,
    /// Stats that require processing
    pub calculated: CalculatedStats,
}

#[derive(Debug, Default, Clone)]
pub struct RawStats {
    /// Total number of log entries processed
    pub total_count: usize,
    /// Number of ERROR log entries processed in the last minute
    pub error_count: usize,
    /// Number of INFO log entries processed in the last minute
    pub info_count: usize,
    /// Number of DEBUG log entries processed in the last minute
    pub debug_count: usize,
}

#[derive(Debug, Default, Clone)]
pub struct CalculatedStats {
    /// Percentage of ERROR log entries
    pub error_pct: f64,
    /// Percentage of INFO log entries
    pub info_pct: f64,
    /// Percentage of DEBUG log entries
    pub debug_pct: f64,
}

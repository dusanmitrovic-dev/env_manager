use crate::logger::SeverityLevel;

/// Configuration parameters for the [Logger].
///
/// # Field
///
/// - [minimal_severity_level]: Minimal logging severity level threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggerConfig {
    pub minimal_severity_level: SeverityLevel,
}

impl LoggerConfig {
    /// Returns a new [LoggerConfig] with default [SeverityLevel::Info].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            minimal_severity_level: SeverityLevel::Info,
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self::new()
    }
}

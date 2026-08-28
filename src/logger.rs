//! # Logger
//!
//! Provides logging features with standard severity levels.
//! Based on: https://en.wikipedia.org/wiki/Syslog.
//!
//! ## Examples
//! ```
//!
//! ```

/// Standard severities of issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SeverityLevel {
    /// System is unusable.
    Emerg = 0,
    /// Action must be taken immediately.
    Alert = 1,
    /// Critical conditions.
    Crit = 2,
    /// Error conditions.
    Err = 3,
    /// Warning conditions.
    Warning = 4,
    /// Normal but significant conditions.
    Notice = 5,
    /// Informational messages.
    Info = 6,
    /// Debug-level messages.
    Debug = 7,
}

impl SeverityLevel {
    /// Returns severity level as uppercase tag string in square brackets.
    #[must_use]
    pub const fn as_tag(self) -> &'static str {
        match self {
            Self::Emerg => "[EMERGENCY]",
            Self::Alert => "[ALERT]",
            Self::Crit => "[CRITICAL]",
            Self::Err => "[ERROR]",
            Self::Warning => "[WARNING]",
            Self::Notice => "[NOTICE]",
            Self::Info => "[INFO]",
            Self::Debug => "[DEBUG]",
        }
    }
}

//! # Logger
//!
//! Provides logging features with standard severity levels.
//! Based on: https://en.wikipedia.org/wiki/Syslog.

use std::io::{self, Write};

use crate::error::LoggerError;

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
    /// Returns severity level as an uppercase tag string in square brackets.
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

/// The maximum allowed length of the log message. Default value to which
/// logger will truncate over the limit log messages.
const MAXIMUM_MESSAGE_LENGTH: usize = 1024;
const TRUNCATION_SUFFIX: &str = "... [TRUNCATED].";
const COLON_SEPARATOR: &str = ": ";
const NEWLINE: &str = "\n";

/// [Logger] structure with standard severity levels and logging feature methods.
///
/// # Fields
///
/// - [minimal_severity_level]: Minimal logging severity level.
pub struct Logger {
    minimal_severity_level: SeverityLevel,
}

impl Logger {
    #[must_use]
    pub const fn new(minimal_severity_level: SeverityLevel) -> Self {
        Self {
            minimal_severity_level,
        }
    }

    /// Logs the [message] with the [severity_level].
    ///
    /// Messages with [severity_level] higher than [self.minimal_severity_level]
    /// will be dropped.
    ///
    /// Messages longer than [MAXIMUM_MESSAGE_LENGTH] will be safely truncated.
    ///
    /// # Returns
    ///
    /// The exact number of bytes written.
    pub fn log(&self, severity_level: SeverityLevel, message: &str) -> Result<usize, LoggerError> {
        if severity_level > self.minimal_severity_level {
            return Ok(0);
        }

        let tag = severity_level.as_tag();

        let total_overhead =
            tag.len() + COLON_SEPARATOR.len() + TRUNCATION_SUFFIX.len() + NEWLINE.len();

        assert!(
            MAXIMUM_MESSAGE_LENGTH > total_overhead,
            "Maximum message length is smaller or equal total overhead."
        );

        let maximum_message_budget = MAXIMUM_MESSAGE_LENGTH - total_overhead;

        let (safe_message, is_truncated) = Self::truncate_log_utf8(message, maximum_message_budget);

        let bytes_written = if (severity_level as u8) <= (SeverityLevel::Warning as u8) {
            let stderr = io::stderr();
            let mut handle = stderr.lock();
            Self::write_and_count(&mut handle, tag, safe_message, is_truncated)?
        } else {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            Self::write_and_count(&mut handle, tag, safe_message, is_truncated)?
        };

        assert!(
            bytes_written > 0,
            "Zero bytes written for non-filtered log."
        );
        assert!(
            bytes_written <= MAXIMUM_MESSAGE_LENGTH,
            "Bytes written exceeded MAXIMUM_MESSAGE_LENGTH."
        );

        Ok(bytes_written)
    }

    fn write_and_count<W: Write>(
        writer: &mut W,
        tag: &'static str,
        message: &str,
        is_truncated: bool,
    ) -> io::Result<usize> {
        assert!(!tag.is_empty(), "Tag cannot be empty.");

        let mut total_bytes = 0;

        writer.write_all(tag.as_bytes())?;
        total_bytes += tag.len();

        assert_eq!(total_bytes, tag.len(), "Write tag total bytes mismatch.");

        writer.write_all(COLON_SEPARATOR.as_bytes())?;
        total_bytes += COLON_SEPARATOR.len();

        assert_eq!(
            total_bytes,
            tag.len() + COLON_SEPARATOR.len(),
            "Write colon separator total bytes mismatch."
        );

        writer.write_all(message.as_bytes())?;
        total_bytes += message.len();

        assert_eq!(
            total_bytes,
            tag.len() + COLON_SEPARATOR.len() + message.len(),
            "Write message total bytes mismatch."
        );

        if is_truncated {
            writer.write_all(TRUNCATION_SUFFIX.as_bytes())?;
            total_bytes += TRUNCATION_SUFFIX.len();

            assert_eq!(
                total_bytes,
                tag.len() + COLON_SEPARATOR.len() + message.len() + TRUNCATION_SUFFIX.len(),
                "Write truncation suffix total bytes mismatch."
            );
        }

        writer.write_all(NEWLINE.as_bytes())?;
        total_bytes += NEWLINE.len();

        const NEWLINE_TOTAL_MISMATCH: &str = "Write newline character total bytes mismatch.";
        if is_truncated {
            assert_eq!(
                total_bytes,
                tag.len()
                    + COLON_SEPARATOR.len()
                    + message.len()
                    + TRUNCATION_SUFFIX.len()
                    + NEWLINE.len(),
                "{}",
                NEWLINE_TOTAL_MISMATCH
            );
        } else {
            assert_eq!(
                total_bytes,
                tag.len() + COLON_SEPARATOR.len() + message.len() + NEWLINE.len(),
                "{}",
                NEWLINE_TOTAL_MISMATCH
            );
        }

        writer.flush()?;

        Ok(total_bytes)
    }

    /// Logs the [message] with [SeverityLevel::Emerg].
    #[inline]
    pub fn emergency(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Emerg, message)
    }

    /// Logs the [message] with [SeverityLevel::Alert].
    #[inline]
    pub fn alert(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Alert, message)
    }

    /// Logs the [message] with [SeverityLevel::Crit].
    #[inline]
    pub fn critical(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Crit, message)
    }

    /// Logs the [message] with [SeverityLevel::Err].
    #[inline]
    pub fn error(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Err, message)
    }

    /// Logs the [message] with [SeverityLevel::Warning].
    #[inline]
    pub fn warning(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Warning, message)
    }

    /// Logs the [message] with [SeverityLevel::Notice].
    #[inline]
    pub fn notice(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Notice, message)
    }

    /// Logs the [message] with [SeverityLevel::Info].
    #[inline]
    pub fn info(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Info, message)
    }

    /// Logs the [message] with [SeverityLevel::Debug].
    #[inline]
    pub fn debug(&self, message: &str) -> Result<usize, LoggerError> {
        self.log(SeverityLevel::Debug, message)
    }

    /// Truncates the [message] if its length breaches [maximum_bytes].
    ///
    /// # Arguments
    ///
    /// - [message]: The log message which might be truncated.
    /// - [maximum_bytes]: The maximum allowed message length and truncation trigger.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - `&str`: The safe, bounded slice.
    /// - `bool`: `true` if truncation occurred, `false` otherwise.
    #[must_use]
    fn truncate_log_utf8(message: &str, maximum_bytes: usize) -> (&str, bool) {
        if message.len() <= maximum_bytes {
            return (message, false);
        }

        let mut boundary = maximum_bytes;
        while boundary > 0 && !message.is_char_boundary(boundary) {
            boundary -= 1;
        }

        assert!(
            boundary <= maximum_bytes,
            "Boundary cannot be longer than maximum_bytes."
        );

        let truncated_message = &message[..boundary];

        assert!(
            truncated_message.len() <= maximum_bytes,
            "Truncated message length cannot be longer than maximum_bytes."
        );

        (truncated_message, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES_WRITTEN_LESS_THAN_ZERO: &str = "Bytes written must be greater than zero.";
    const TRUNCATION_FLAG_MUST_BE_TRUE: &str = "Truncation flag must be true.";

    /// Tests normal logger functionality. The bytes written must be equal to
    /// severity level tag length + message length + colon-space separator
    /// length (2) + newline character length (1).
    #[test]
    fn test_logger_normal_functionality() {
        const DEBUG_MESSAGE: &str = "Hello, debug!";

        let logger = Logger::new(SeverityLevel::Debug);

        _ = logger.emergency("Hello, emergency!");
        _ = logger.alert("Hello, alert!");
        _ = logger.critical("Hello, critical!");
        _ = logger.error("Hello, error!");
        _ = logger.warning("Hello, warning!");
        _ = logger.notice("Hello, notice!");
        _ = logger.info("Hello, info!");
        let debug_result = logger.debug(DEBUG_MESSAGE);

        let bytes_written = match debug_result {
            Ok(bytes_written) => bytes_written,
            Err(error) => panic!("{}", error),
        };

        assert!(bytes_written > 0, "{}", BYTES_WRITTEN_LESS_THAN_ZERO);
        assert_eq!(
            bytes_written,
            SeverityLevel::Debug.as_tag().len()
                + DEBUG_MESSAGE.len()
                + COLON_SEPARATOR.len()
                + NEWLINE.len()
        );
    }

    /// Tests the ability for logger to drop messages above the minimal
    /// logger severity level.
    #[test]
    fn test_logger_message_filtering() {
        let logger = Logger::new(SeverityLevel::Info);

        _ = logger.info("Hello, info!");
        let bytes_written = match logger.debug("Hello, debug!") {
            Ok(bytes_written) => bytes_written,
            Err(error) => {
                panic!("{}", error);
            }
        };

        assert_eq!(
            bytes_written, 0,
            "Debug log message must be dropped on SeverityLevel::Info."
        );
    }

    /// Tests the total output line length never exceeds [MAXIMUM_MESSAGE_LENGTH].
    #[test]
    fn test_logger_output_length() {
        let logger = Logger::new(SeverityLevel::Debug);
        let huge_message = "A".repeat(2000);

        let bytes_written = match logger.emergency(&huge_message) {
            Ok(bytes_written) => bytes_written,
            Err(error) => {
                panic!("{}", error);
            }
        };

        assert!(bytes_written > 0, "{}", BYTES_WRITTEN_LESS_THAN_ZERO);
        assert!(
            bytes_written <= MAXIMUM_MESSAGE_LENGTH,
            "Output line was {} bytes, exceeded {}",
            bytes_written,
            MAXIMUM_MESSAGE_LENGTH
        );
    }

    /// Tests that log messages longer than the [MAXIMUM_MESSAGE_LENGTH] are
    /// properly truncated.
    #[test]
    fn test_logger_message_truncation() {
        let huge_message = "A".repeat(2000);

        let (truncated_message, was_truncated) =
            Logger::truncate_log_utf8(&huge_message, MAXIMUM_MESSAGE_LENGTH);

        assert!(was_truncated, "{}", TRUNCATION_FLAG_MUST_BE_TRUE);
        assert_eq!(
            truncated_message.len(),
            MAXIMUM_MESSAGE_LENGTH,
            "Truncated message length must be exactly equal the budget limit."
        );
    }

    /// Tests that multi-byte UTF-8 characters, like emojis, are never cut in
    /// half. "🦀" Ferris is 4 bytes. 3 crabs = 12 bytes in total. If we truncate
    /// at 6 bytes, in the middle of the second crab, it must safely fall back to
    /// 4 bytes (only 1 crab).
    #[test]
    fn test_logger_message_utf8_multibyte_safety() {
        let crab_message = "🦀🦀🦀";

        let (truncated_message, was_truncated) = Logger::truncate_log_utf8(crab_message, 6);

        assert!(was_truncated, "{}", TRUNCATION_FLAG_MUST_BE_TRUE);
        assert_eq!(
            truncated_message, "🦀",
            "Truncated message must have exactly one Ferris crab."
        );
    }
}

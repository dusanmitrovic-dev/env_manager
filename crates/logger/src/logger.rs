//! # Logger
//!
//! Provides logging features with standard severity levels.
//! Based on: https://en.wikipedia.org/wiki/Syslog.

use std::fmt;
use std::io::{self, Write};

use crate::builder::LoggerBuilder;
use crate::config::LoggerConfig;
use crate::error::LoggerError;
use crate::fixed_buffer::FixedBuffer;
use crate::severity_level::SeverityLevel;

/// The maximum allowed length of the log message.
///
/// Messages exceeding this limit will be safely truncated.
const MAXIMUM_MESSAGE_LENGTH: usize = 1024;
/// Suffix used on messages which exceed maximum message length.
const TRUNCATION_SUFFIX: &str = "... [TRUNCATED].";
/// Separator used between severity level tags and log messages.
const COLON_SEPARATOR: &str = ": ";
/// Newline character appended to terminate each log frame.
const NEWLINE: &str = "\n";
/// Compile-time constant representing the fixed formatting overhead in log messages.
const STATIC_OVERHEAD: usize = COLON_SEPARATOR.len() + TRUNCATION_SUFFIX.len() + NEWLINE.len();

/// [Logger] structure with standard severity levels and logging feature methods.
///
/// # Fields
///
/// - [config]: The configuration parameters of this logger.
pub struct Logger {
    config: LoggerConfig,
}

impl Logger {
    /// Creates a new [Logger] with a specific [SeverityLevel].
    #[must_use]
    pub const fn new(minimal_severity_level: SeverityLevel) -> Self {
        Self {
            config: LoggerConfig {
                minimal_severity_level,
            },
        }
    }

    /// Creates a [Logger] directly from a [LoggerConfig] object.
    ///
    /// # Arguments
    ///
    /// - [config]: The configuration object to initialize the logger with.
    #[must_use]
    pub const fn from_config(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Returns a [LoggerBuilder] for configurable initialization.
    #[must_use]
    pub const fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Returns the configured minimal severity level threshold.
    #[must_use]
    #[inline]
    pub const fn minimal_severity_level(&self) -> SeverityLevel {
        self.config.minimal_severity_level
    }

    /// Checks if a severity level is enabled under the current threshold.
    ///
    /// # Arguments
    ///
    /// - [severity_level]: The level to evaluate against the configured threshold.
    #[must_use]
    #[inline]
    pub fn is_enabled(&self, severity_level: SeverityLevel) -> bool {
        (severity_level as u8) <= (self.config.minimal_severity_level as u8)
    }

    /// Logs lazily evaluated formatting arguments.
    ///
    /// Exits early with `Ok(0)` before executing any formatting if the severity
    /// is disabled.
    ///
    /// # Arguments
    ///
    /// - [severity_level]: Severity level of the log message.
    /// - [arguments]: Lazily captured format arguments.
    ///
    /// # Errors
    ///
    /// - [LoggerError::Format]: On formatting errors.
    /// - [LoggerError::Io]: On write failures.
    pub fn log_args(
        &self,
        severity_level: SeverityLevel,
        arguments: fmt::Arguments<'_>,
    ) -> Result<usize, LoggerError> {
        if !self.is_enabled(severity_level) {
            return Ok(0);
        }

        let mut fixed_buffer = FixedBuffer::<MAXIMUM_MESSAGE_LENGTH>::new();

        fmt::write(&mut fixed_buffer, arguments)?;

        self.log(severity_level, fixed_buffer.as_str())
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
        if !self.is_enabled(severity_level) {
            return Ok(0);
        }

        let tag = severity_level.as_tag();

        let (safe_message, is_truncated) = Self::prepare_payload(tag, message);

        Self::dispatch_output(severity_level, tag, safe_message, is_truncated)
    }

    /// Calculates formatting overhead, asserts budget invariants, and safely
    /// truncates the payload.
    ///
    /// # Arguments
    ///
    /// - [tag]: Severity level tag string.
    /// - [message]: Raw message slice.
    ///
    /// # Returns
    ///
    /// A tuple containing the bounded message slice and a boolean indicating
    /// if the truncation occurred.
    fn prepare_payload<'a>(tag: &'static str, message: &'a str) -> (&'a str, bool) {
        let total_overhead = tag.len() + STATIC_OVERHEAD;

        assert!(
            MAXIMUM_MESSAGE_LENGTH > total_overhead,
            "Maximum message length is smaller or equal total overhead."
        );

        let maximum_message_budget = MAXIMUM_MESSAGE_LENGTH - total_overhead;

        let (safe_message, is_truncated) = Self::truncate_log_utf8(message, maximum_message_budget);

        assert!(
            safe_message.len() <= maximum_message_budget,
            "Payload length exceeded calculated budget."
        );

        (safe_message, is_truncated)
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

    /// Routes the formatted payload to the appropriate terminal stream and
    /// verifies output bounds.
    ///
    /// # Arguments
    ///
    /// - [severity_level]: Severity level determining stream routing.
    /// - [tag]: Severity level tag string.
    /// - [message]: Bounded message slice.
    /// - [is_truncated]: Flag indicating if message exceeded budget.
    ///
    /// # Returns
    ///
    /// The exact number of bytes written to the stream, or `Ok(0)` on broken
    /// pipe.
    ///
    /// # Errors
    ///
    /// Returns [LoggerError::Io] if an unrecoverable terminal I/O failure
    /// occurs.
    fn dispatch_output(
        severity_level: SeverityLevel,
        tag: &'static str,
        message: &str,
        is_truncated: bool,
    ) -> Result<usize, LoggerError> {
        assert!(
            message.len() <= MAXIMUM_MESSAGE_LENGTH,
            "Incoming message payload exceeded MAXIMUM_MESSAGE_LENGTH."
        );

        let mut stderr_handle;
        let mut stdout_handle;

        let writer: &mut dyn Write = if severity_level.is_stderr() {
            stderr_handle = io::stderr().lock();

            &mut stderr_handle
        } else {
            stdout_handle = io::stdout().lock();

            &mut stdout_handle
        };

        let bytes_written = Self::write_log_frame(writer, tag, message, is_truncated)?;

        if bytes_written == 0 {
            return Ok(0);
        }

        assert!(
            bytes_written <= MAXIMUM_MESSAGE_LENGTH,
            "Bytes written exceeded MAXIMUM_MESSAGE_LENGTH."
        );

        Ok(bytes_written)
    }

    /// Fault-tolerant writer that handles [io::ErrorKind::BrokenPipe].
    ///
    /// # Arguments
    ///
    /// - [writer]: Target I/O writer.
    /// - [tag]: Severity level tag string.
    /// - [message]: Bounded message slice.
    /// - [is_truncated]: Flag indicating if message exceeded budget.
    ///
    /// # Returns
    ///
    /// The exact number of bytes written, or `Ok(0)` if down stream pipe was
    /// broken.
    ///
    /// # Errors
    ///
    /// Returns [LoggerError::Io] on unrecoverable terminal write failures.
    fn write_log_frame(
        writer: &mut dyn Write,
        tag: &'static str,
        message: &str,
        is_truncated: bool,
    ) -> Result<usize, LoggerError> {
        match Self::write_stream(writer, tag, message, is_truncated) {
            Ok(bytes) => Ok(bytes),
            Err(ref err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(0),
            Err(err) => Err(LoggerError::Io(err)),
        }
    }

    /// Writes the sequential frame slices to the stream and returns exact
    /// bytes written.
    ///
    /// # Arguments
    ///
    /// - [writer]: Target I/O writer.
    /// - [tag]: Severity level tag string.
    /// - [message]: Bounded message slice.
    /// - [is_truncated]: Flag indicating if message exceeded budget.
    ///
    /// # Returns
    ///
    /// The exact number of bytes sequentially written.
    ///
    /// # Errors
    ///
    /// Returns [io::Error] if any underlying slice fails to write or flush.
    fn write_stream(
        writer: &mut dyn Write,
        tag: &'static str,
        message: &str,
        is_truncated: bool,
    ) -> io::Result<usize> {
        assert!(!tag.is_empty(), "Tag cannot be empty.");

        let mut total_bytes: usize = 0;

        total_bytes += Self::write_slice(writer, tag.as_bytes())?;
        total_bytes += Self::write_slice(writer, COLON_SEPARATOR.as_bytes())?;
        total_bytes += Self::write_slice(writer, message.as_bytes())?;

        if is_truncated {
            total_bytes += Self::write_slice(writer, TRUNCATION_SUFFIX.as_bytes())?;
        }

        total_bytes += Self::write_slice(writer, NEWLINE.as_bytes())?;

        assert!(
            total_bytes > 0,
            "Successfully formatted stream yielded zero bytes."
        );

        writer.flush()?;

        Ok(total_bytes)
    }

    /// Writes a single byte slice to the writer and returns the exact number
    /// of bytes written.
    ///
    /// # Arguments
    ///
    /// - [writer]: Target I/O writer.
    /// - [bytes]: Raw byte slice to write.
    ///
    /// # Errors
    ///
    /// Returns [io::Error] if the underlying stream rejects the write.
    fn write_slice(writer: &mut dyn Write, bytes: &[u8]) -> io::Result<usize> {
        writer.write_all(bytes)?;

        Ok(bytes.len())
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

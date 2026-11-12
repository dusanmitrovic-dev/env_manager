use crate::config::LoggerConfig;
use crate::error::LoggerError;
use crate::global;
use crate::logger::Logger;
use crate::severity_level::SeverityLevel;

/// A configurable builder for constructing [Logger] instances.
///
/// # Fields
///
/// - [config]: The encapsulated logger configuration state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    /// Returns a new [LoggerBuilder] initialized with default logger settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: LoggerConfig::new(),
        }
    }

    /// Sets the minimal severity level threshold.
    #[must_use]
    pub const fn with_minimal_severity_level(mut self, level: SeverityLevel) -> Self {
        self.config.minimal_severity_level = level;

        self
    }

    /// Builds an isolated, local [Logger] instance.
    #[must_use]
    pub const fn build(self) -> Logger {
        Logger::from_config(self.config)
    }

    /// Builds and registers a logger as the global singleton.
    ///
    /// # Returns
    ///
    /// A direct reference to the newly initialized static [Logger].
    ///
    /// # Errors
    ///
    /// Returns [LoggerError::AlreadyInitialized] if the global logger has
    /// already been set.
    pub fn initialize(self) -> Result<&'static Logger, LoggerError> {
        global::initialize_with_logger(self.build())
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        LoggerBuilder::new()
    }
}

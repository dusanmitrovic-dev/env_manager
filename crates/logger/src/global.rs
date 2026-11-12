use std::sync::OnceLock;

use crate::error::LoggerError;
use crate::logger::Logger;
use crate::severity_level::SeverityLevel;

/// Fallback logger used when uninitialized logs fire before [initialize()].
static FALLBACK_LOGGER: Logger = Logger::new(SeverityLevel::Info);
/// The default global logger implemented as an immutable write-once singleton.
static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initializes the global logger with default settings of [SeverityLevel::Info].
///
/// # Returns
///
/// A direct reference to the newly initialized static [Logger].
///
/// # Errors
///
/// Returns [LoggerError::AlreadyInitialized] if the global logger has already
/// been set.
pub fn initialize() -> Result<&'static Logger, LoggerError> {
    let static_logger = initialize_with_logger(Logger::new(SeverityLevel::Info))?;

    assert_eq!(
        static_logger.minimal_severity_level(),
        SeverityLevel::Info,
        "Default initialization failed to set SeverityLevel::Info."
    );

    Ok(static_logger)
}

/// Initializes the global logger with a specific [Logger] instance.
///
/// # Arguments
///
/// - [logger]: The configured [Logger] instance to register globally.
///
/// # Returns
///
/// A direct reference to the newly initialized static [Logger].
///
/// # Errors
///
/// Returns [LoggerError::AlreadyInitialized] if the global logger has already
/// been set.
pub fn initialize_with_logger(logger: Logger) -> Result<&'static Logger, LoggerError> {
    let set_result = GLOBAL_LOGGER.set(logger);

    match set_result {
        Ok(()) => {
            let cell_logger = GLOBAL_LOGGER
                .get()
                .expect("OnceLock reported Ok, but cell is empty.");

            let static_logger = global_logger();

            assert_eq!(
                static_logger.minimal_severity_level(),
                cell_logger.minimal_severity_level(),
                "Returned logger does not match cell state."
            );

            Ok(static_logger)
        }
        Err(_) => Err(LoggerError::AlreadyInitialized),
    }
}

/// Internal accessor for the global logger.
///
/// Falls back to the [FALLBACK_LOGGER] without setting the cell if uninitialized.
#[doc(hidden)]
#[must_use]
pub fn global_logger() -> &'static Logger {
    match GLOBAL_LOGGER.get() {
        Some(logger) => logger,
        None => &FALLBACK_LOGGER,
    }
}

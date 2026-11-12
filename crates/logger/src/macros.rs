/// Logs a lazily evaluated message with [SeverityLevel::Emerg].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! emergency {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Emerg) {
                logger.log_args(
                    $crate::SeverityLevel::Emerg,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Emerg) {
            $logger.log_args(
                $crate::SeverityLevel::Emerg,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Emerg) {
            $logger.log_args(
                $crate::SeverityLevel::Emerg,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Alert].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! alert {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Alert) {
                logger.log_args(
                    $crate::SeverityLevel::Alert,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Alert) {
            $logger.log_args(
                $crate::SeverityLevel::Alert,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Alert) {
            $logger.log_args(
                $crate::SeverityLevel::Alert,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Crit].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! critical {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Crit) {
                logger.log_args(
                    $crate::SeverityLevel::Crit,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Crit) {
            $logger.log_args(
                $crate::SeverityLevel::Crit,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Crit) {
            $logger.log_args(
                $crate::SeverityLevel::Crit,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Err].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Err) {
                logger.log_args(
                    $crate::SeverityLevel::Err,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Err) {
            $logger.log_args(
                $crate::SeverityLevel::Err,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Err) {
            $logger.log_args(
                $crate::SeverityLevel::Err,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Warning].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! warning {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Warning) {
                logger.log_args(
                    $crate::SeverityLevel::Warning,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Warning) {
            $logger.log_args(
                $crate::SeverityLevel::Warning,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Warning) {
            $logger.log_args(
                $crate::SeverityLevel::Warning,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Notice].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! notice {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Notice) {
                logger.log_args(
                    $crate::SeverityLevel::Notice,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Notice) {
            $logger.log_args(
                $crate::SeverityLevel::Notice,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Notice) {
            $logger.log_args(
                $crate::SeverityLevel::Notice,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Info].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Info) {
                logger.log_args(
                    $crate::SeverityLevel::Info,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Info) {
            $logger.log_args(
                $crate::SeverityLevel::Info,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Info) {
            $logger.log_args(
                $crate::SeverityLevel::Info,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

/// Logs a lazily evaluated message with [SeverityLevel::Debug].
///
/// Formatting is skipped entirely if the severity level is disabled.
#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            let logger = $crate::global::global_logger();

            if logger.is_enabled($crate::SeverityLevel::Debug) {
                logger.log_args(
                    $crate::SeverityLevel::Debug,
                    format_args!($fmt $(, $($arg)+)?),
                )
            } else {
                Ok(0)
            }
        }
    };
    (target: $logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Debug) {
            $logger.log_args(
                $crate::SeverityLevel::Debug,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
    ($logger:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $logger.is_enabled($crate::SeverityLevel::Debug) {
            $logger.log_args(
                $crate::SeverityLevel::Debug,
                format_args!($fmt $(, $($arg)+)?),
            )
        } else {
            Ok(0)
        }
    };
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::Logger;
    use crate::LoggerError;
    use crate::SeverityLevel;

    const TEST_LOGGER: Logger = Logger::new(SeverityLevel::Warning);

    /// Helper mock function which returns a mock string to be printed.
    fn side_effect_calculation(counter: &AtomicUsize) -> &'static str {
        counter.fetch_add(1, Ordering::SeqCst);

        "calculated"
    }

    /// Tests that target macro skips argument evaluation when severity is
    /// disabled.
    #[test]
    fn test_target_macro_skips_disabled_level_evaluation() -> Result<(), LoggerError> {
        let counter = AtomicUsize::new(0);

        let bytes_written = debug!(TEST_LOGGER, "Value: {}", side_effect_calculation(&counter))?;

        assert_eq!(
            bytes_written, 0,
            "Target debug log message must be dropped on disabled SeverityLevel::Warning."
        );
        assert_eq!(
            counter.load(Ordering::SeqCst),
            0,
            "Target debug must skip argument evaluation on disabled level SeverityLevel::Warning."
        );

        Ok(())
    }

    /// Tests that target macro evaluates arguments when severity is enabled.
    #[test]
    fn test_target_macro_evaluates_enabled_level_args() -> Result<(), LoggerError> {
        let counter = AtomicUsize::new(0);

        let bytes_written = warning!(TEST_LOGGER, "Value: {}", side_effect_calculation(&counter))?;

        assert!(
            bytes_written > 0,
            "Target warning log message must write bytes on enabled SeverityLevel::Warning."
        );
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "Target warning must evaluate arguments on enabled SeverityLevel::Warning."
        );

        Ok(())
    }

    /// Test that macro skips argument evaluation when severity level is
    /// disabled.
    #[test]
    fn test_macro_skips_disabled_level_evaluation() -> Result<(), LoggerError> {
        let counter = AtomicUsize::new(0);

        let bytes_written = debug!("Value: {}", side_effect_calculation(&counter))?;

        assert_eq!(
            bytes_written, 0,
            "Debug log message must be dropped on disabled fallback SeverityLevel::Info."
        );
        assert_eq!(
            counter.load(Ordering::SeqCst),
            0,
            "Debug must skip argument evaluation on disabled fallback SeverityLevel::Info."
        );

        Ok(())
    }

    /// Tests that macro evaluates arguments when severity level is enabled.
    #[test]
    fn test_macro_evaluates_enabled_level_args() -> Result<(), LoggerError> {
        let counter = AtomicUsize::new(0);

        let bytes_written = info!("Value: {}", side_effect_calculation(&counter))?;

        assert!(
            bytes_written > 0,
            "Info log message must write bytes on enabled fallback SeverityLevel::Info."
        );
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "Info must evaluate arguments on enabled fallback SeverityLevel::Info."
        );

        Ok(())
    }

    /// Tests single string literals without formatting arguments for targeted
    /// logger.
    #[test]
    fn test_target_single_literal_without_args() -> Result<(), LoggerError> {
        let bytes_written = warning!(TEST_LOGGER, "Single target string literal test.")?;

        assert!(
            bytes_written > 0,
            "Target warning single literal message must write bytes on enabled SeverityLevel::Warning."
        );

        Ok(())
    }

    /// Tests single string literals without formatting arguments for the global
    /// logger.
    #[test]
    fn test_single_literal_without_args() -> Result<(), LoggerError> {
        let bytes_written = info!("Single string literal test.")?;

        assert!(
            bytes_written > 0,
            "Info single message must write bytes on enabled fallback SeverityLevel::Info."
        );

        Ok(())
    }
}

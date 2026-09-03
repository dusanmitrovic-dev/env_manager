#[macro_export]
macro_rules! emergency {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Emerg) {
            $logger.log_args($crate::SeverityLevel::Emerg, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! alert {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Alert) {
            $logger.log_args($crate::SeverityLevel::Alert, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! critical {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Crit) {
            $logger.log_args($crate::SeverityLevel::Crit, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Err) {
            $logger.log_args($crate::SeverityLevel::Err, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! warning {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Warning) {
            $logger.log_args($crate::SeverityLevel::Warning, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! notice {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Notice) {
            $logger.log_args($crate::SeverityLevel::Notice, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Info) {
            $logger.log_args($crate::SeverityLevel::Info, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)+) => {
        if $logger.is_enabled($crate::SeverityLevel::Debug) {
            $logger.log_args($crate::SeverityLevel::Debug, format_args!($($arg)+))
        } else {
            Ok(0)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Logger;

    use std::sync::atomic::{AtomicUsize, Ordering};

    static LAZY_EVAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn side_effect_calculation() -> &'static str {
        LAZY_EVAL_COUNTER.fetch_add(1, Ordering::SeqCst);

        "calculated"
    }

    /// Tests that arguments in macros are never evaluated when the severity
    /// level is disabled.
    #[test]
    fn test_logger_lazy_macro_evaluation() {
        LAZY_EVAL_COUNTER.store(0, Ordering::SeqCst);

        let logger = Logger::new(crate::SeverityLevel::Warning);

        let result = debug!(logger, "Value: {}", side_effect_calculation());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        assert_eq!(
            LAZY_EVAL_COUNTER.load(Ordering::SeqCst),
            0,
            "Lazy evaluation failed, expression was evaluated for a disabled log level."
        );

        let result = warning!(logger, "Value: {}", side_effect_calculation());

        assert!(result.is_ok());
        assert!(result.unwrap() > 0);

        assert_eq!(
            LAZY_EVAL_COUNTER.load(Ordering::SeqCst),
            1,
            "Expression was not evaluated for an enabled log."
        );
    }
}

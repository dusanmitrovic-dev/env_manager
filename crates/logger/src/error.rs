use core::fmt;
use std::error;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
#[must_use]
pub enum LoggerError {
    Io(io::Error),
    Format(fmt::Error),
}

impl From<io::Error> for LoggerError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<fmt::Error> for LoggerError {
    fn from(error: fmt::Error) -> Self {
        Self::Format(error)
    }
}

impl Display for LoggerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "I/O error: {error}")
            }
            Self::Format(error) => {
                write!(formatter, "Formatting error: {error}")
            }
        }
    }
}

impl error::Error for LoggerError {}

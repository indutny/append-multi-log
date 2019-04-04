extern crate append_log;

use std::error::Error as StdError;
use std::fmt;
use std::io;

use append_log::{Error as LogError};

#[derive(Debug)]
pub enum Error {
    Log(LogError),
    IO(io::Error),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Log(_) => "Log error",
            Error::IO(_) => "IO error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Log(err) => write!(f, "Log error: {}", err),
            Error::IO(err) => write!(f, "IO error: {}", err),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<LogError> for Error {
    fn from(err: LogError) -> Self {
        Error::Log(err)
    }
}

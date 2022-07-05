use std::{io, num};

/// Errors that can occur while using the grab CLI.
#[derive(Debug)]
pub enum CliError {
    /// I/O error
    Io(io::Error),
    /// Error in parsing context number
    Parse(num::ParseIntError),
    /// Error in compiling regex
    Regex(regex::Error),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> Self {
        CliError::Parse(err)
    }
}

impl From<regex::Error> for CliError {
    fn from(err: regex::Error) -> Self {
        CliError::Regex(err)
    }
}

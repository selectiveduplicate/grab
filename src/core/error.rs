use std::{fmt::Display, io, num};

/// Writes to the standard error stream and terminates the current process.
#[macro_export]
macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

/// Errors that can occur while using the CLI.
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

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Parse(ref err) => err.fmt(f),
            CliError::Regex(ref err) => err.fmt(f),
        }
    }
}

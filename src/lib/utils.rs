use crate::lib::error::CliError;
use colored::Colorize;
use regex::RegexBuilder;

/// Writes to the standard error stream and terminates the current process.
#[macro_export]
macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

/// Creates a new `BufWriter` object to write to the standard output stream.
#[macro_export]
macro_rules! getwriter {
    () => {
        {
            let stdout = std::io::stdout();
            let handle = stdout.lock();
            let writer = std::io::BufWriter::new(handle);
            writer
        }
    };
}

/// Contains colors to apply to patterns like group separators and matches
pub enum Colors {
    Red,
    Green,
    Blue,
}

impl Colors {
    /// Colorizes a `pattern`. Based on the variant of `Colors` provided, it
    /// calls the appropriate function of the `Colorize` trait.
    pub fn colorize_pattern(color: Self, pattern: &str) -> String {
        match color {
            Self::Red => pattern.red().to_string(),
            Self::Green => pattern.green().to_string(),
            Self::Blue => pattern.blue().to_string(),
        }
    }
}

/// Parses the context number.
pub fn parse_context_number(n: Option<&str>) -> Result<usize, CliError> {
    n.unwrap().parse::<usize>().map_err(|err| err.into())
}

/// Compiles the regular expression given by `p`.
pub fn compile_regex(p: &str, is_case_insensitive: bool) -> Result<regex::Regex, CliError> {
    let re = match is_case_insensitive {
        true => RegexBuilder::new(p).case_insensitive(true).build()?,
        false => RegexBuilder::new(p).build()?,
    };
    Ok(re)
}

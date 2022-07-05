use crate::lib::error::CliError;
use colored::Colorize;

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

/// Parses the context number, returning a `usize` upon success.
/// Exits the program with exit code 1 if the argument is not an integer.
pub fn parse_context_number(n: Option<&str>) -> Result<usize, CliError> {
    let ctx_number = n.unwrap().parse()?;
    Ok(ctx_number)
}

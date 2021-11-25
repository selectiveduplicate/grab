use colored::Colorize;
use std::num::ParseIntError;

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

/// Error handler function for `parse_context_number`.
fn exit_on_invalid_context(err: ParseIntError) {
    eprintln!("{}: context length argument must be an integer.", err);
    std::process::exit(1);
}

/// Parses the context number, returning a `usize` upon success.
/// Exits the program with exit code 1 if the argument is not an integer.
pub fn parse_context_number(n: Option<&str>) -> usize {
    n.unwrap()
        .parse::<usize>()
        .map_err(exit_on_invalid_context)
        .unwrap()
}

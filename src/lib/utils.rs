use colored::Colorize;

// Contains colors to apply to patterns like group separators and matches
pub enum Colors {
    Red,
    Green,
}

impl Colors {
    // Colorizes a `pattern`. Based on the variant of `Colors` provided, it
    // calls the appropriate function of the `Colorize` trait.
    pub fn colorize_pattern(color: Self, pattern: &str) -> String {
        match color {
            Self::Red => pattern.red().to_string(),
            Self::Green => pattern.green().to_string(),
        }
    }
}
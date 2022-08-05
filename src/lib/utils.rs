use crate::fatal;
use crate::lib::error::CliError;
use colored::Colorize;
use regex::RegexBuilder;

/// Creates a new `BufWriter` object to write to the standard output stream.
#[macro_export]
macro_rules! getwriter {
    () => {{
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        let writer = std::io::BufWriter::new(handle);
        writer
    }};
}

/// Contains colors to apply to patterns like group separators and matches
pub(crate) enum Colors {
    Red,
    Green,
    Blue,
}

impl Colors {
    /// Colorizes a `pattern`. Based on the variant of `Colors` provided, it
    /// calls the appropriate function of the `Colorize` trait.
    pub(crate) fn colorize_pattern(color: Self, pattern: &str) -> String {
        match color {
            Self::Red => pattern.red().to_string(),
            Self::Green => pattern.green().to_string(),
            Self::Blue => pattern.blue().to_string(),
        }
    }
}

/// Represents the type of context lines.
#[derive(Clone, Copy)]
pub(crate) enum ContextKind {
    /// Trailing context
    After(usize),
    /// Leading context
    Before(usize),
    /// Both trailing and leading
    AfterAndBefore(usize),
    /// No context
    None,
}

/// Tries to parse the context number.
pub(crate) fn parse_context_number(ctx: &str) -> usize {
    ctx.parse::<usize>()
        .map_or_else(|err| fatal!("error: {err}"), |ctx| ctx)
}

/// Compiles the regular expression given by `p`.
pub(crate) fn compile_regex(p: &str, is_case_insensitive: bool) -> Result<regex::Regex, CliError> {
    let re = match is_case_insensitive {
        true => RegexBuilder::new(p).case_insensitive(true).build()?,
        false => RegexBuilder::new(p).build()?,
    };
    Ok(re)
}

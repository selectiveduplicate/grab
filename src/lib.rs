//! Searches for patterns. Prints to the standard output after successful match.
use colored::Colorize;
use regex::Regex;
use std::io::prelude::*;

/// Struct representting the argument flags.
///
/// # Respective flags represented by the fields:
/// ```
/// --count, -c
/// --line-number, -n
/// --color
/// ```
pub struct Flags {
    pub count: bool,
    pub line_number: bool,
    pub colorize: bool,
}

/// Calculates the number of matches found according to the regex pattern and returns it
fn count_matches<T: BufRead + Sized>(reader: T, re: Regex) -> u32 {
    let mut matches: u32 = 0;
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => matches += 1,
            _ => (),
        }
    }
    matches
}

/// Checks status of the `count` field in Flags struct
/// If it's set, then display number of matches returned by calling `count_matches`
/// Otherwise, call `print_matches`
pub fn choose_process<T: BufRead + Sized>(reader: T, re: Regex, flags: &Flags) {
    match flags.count {
        // supress normal output and only print total number of matches found
        true => println!("{}", count_matches(reader, re)),
        // call `print_matches` to print matching lines
        false => print_matches(reader, re, flags),
    }
}

/// Uses the `colored` crate to colorize the passed pattern
fn colorize_pattern(pattern: &str) -> String {
    pattern.red().to_string()
}

/// Prints the lines containing the matches found.
/// Based on the status of the `line_number` field of Flag struct,
/// also prints the 1-based line number preceeding each line.
fn print_matches<T: BufRead + Sized>(reader: T, re: Regex, flags: &Flags) {
    // `.lines()` returns an iterator over each line of `reader`, in the form of `io::Result::String`
    // So a line would be an instance like this: `Ok(line)`
    // `enumerate` gives us the (index, value) pair
    let mut lines = reader.lines().enumerate();

    // `.next()` on an iterator returns the item wrapped in an Option
    // So Each `Some` variant of that option will hold the (index, value) pair
    while let Some((i, Ok(line))) = lines.next() {
        let pattern = match re.find(&line) {
            // `re.find()` returns the byte range holding the first match
            // as_str() returns that match in text form
            Some(pattern) => pattern.as_str(),
            None => continue,
        };
        match (flags.line_number, flags.colorize) {
            (true, false) => println!("{}: {}", i + 1, line),
            (false, true) => println!("{}", re.replace_all(&line, colorize_pattern(pattern))),
            (true, true) => println!(
                "{}: {}",
                i + 1,
                re.replace_all(&line, colorize_pattern(pattern))
            ),
            _ => println!("{}", line),
        }
    }
}

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
/// --ignore-case, -i
/// --after-context, -A,
/// ```
pub struct Flags {
    pub count: bool,
    pub line_number: bool,
    pub colorize: bool,
    pub ignore_case: bool,
    pub after_context: bool,
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

/// Prints trailing context lines with or without line numbers.
/// Each match and its trailing context is separated by the "---" string.
fn print_with_after_context<T: BufRead + Sized>(
    reader: &mut T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
) {
    // For all the matched words/terms
    let mut patterns: Vec<String> = Vec::new();

    // We need to iterate over the `reader` content twice, which is not possible so
    // we move them to a Vector that we can iterate over more than once.
    let mut lines: Vec<String> = Vec::new();
    for line_ in reader.lines() {
        let line = line_.unwrap();
        lines.push(line);
    }

    // For line numbers where matches occur
    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    // Stores each matching line and line number as a tuple Vector
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        match re.find(&line_) {
            Some(pattern) => patterns.push(pattern.as_str().to_string()),
            None => continue,
        }

        matched_line_numbers.push(i);
        let v = Vec::with_capacity(context_number + 1);
        matched_lines_with_number.push(v);
    }

    // We need to iterate `matched_number` of times
    // to find find trailing context lines for each.
    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            let upper_bound = matched_number + context_number;
            if (i >= *matched_number) && (i <= upper_bound) {
                match ((i == *matched_number), flags.colorize) {
                    (true, true) => matched_lines_with_number[j].push((
                        i,
                        re.replace_all(&line, colorize_pattern(&patterns[j]))
                            .to_string(),
                    )),
                    (true, _) => matched_lines_with_number[j].push((i, line.clone())),
                    _ => matched_lines_with_number[j].push((i, line.clone())),
                }
            }
        }
    }

    // Prints matches with context lines.
    // Each group is separated by a hardcoded "---" string.
    // TODO: Allow user to specify a custom separator of choice.
    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len() - 1, index == 0))
    {
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => println!("---"),
        }
        for (i, line) in matched_line.iter() {
            match flags.line_number {
                true => println!("{}: {}", i + 1, line),
                _ => println!("{}", line),
            }
        }
    }
}

/// Checks status of the `count` field in Flags struct.
/// If it's set, then display number of matches returned by calling `count_matches`.
/// Otherwise, call `print_matches`.
pub fn choose_process<T: BufRead + Sized>(
    mut reader: T,
    re: Regex,
    flags: &Flags,
    after_context_number: &str,
) {
    match (flags.count, flags.after_context) {
        (true, _) => println!("{}", count_matches(reader, re)),
        (false, false) => print_matches(reader, re, flags),
        (false, true) => print_with_after_context(
            &mut reader,
            re,
            flags,
            after_context_number.parse::<usize>().unwrap(),
        ),
    }
}

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

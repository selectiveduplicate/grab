use colored::Colorize;
use regex::Regex;
use std::io::prelude::*;

use crate::lib::flag::Flags;

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
    group_separator: &str,
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
    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => println!("{}", group_separator)
        }
        for (i, line) in matched_line.iter() {
            match flags.line_number {
                true => println!("{}: {}", i + 1, line),
                _ => println!("{}", line),
            }
        }
    }
}

/// Prints leading context lines with or without line numbers.
/// Each match and its trailing context is separated by the "---" string.
fn print_with_before_context<T: BufRead + Sized>(
    reader: &mut T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
    group_separator: &str,
) {
    let mut patterns: Vec<String> = Vec::new();

    let mut lines: Vec<String> = Vec::new();
    for line_ in reader.lines() {
        let line = line_.unwrap();
        lines.push(line);
    }

    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
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

    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            // starting point is always positive
            // this handles the case where a match exists on the first line
            let starting_point = matched_number.saturating_sub(context_number);
            if (i >= starting_point) && (i <= *matched_number) {
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

    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => println!("{}", group_separator),
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
    before_context_number: &str,
    group_separator: &str,
) {
    match (flags.count, flags.after_context, flags.before_context) {
        (true, _, _) => println!("{}", count_matches(reader, re)),
        (false, false, false) => print_matches(reader, re, flags),
        (false, true, _) => print_with_after_context(
            &mut reader,
            re,
            flags,
            after_context_number.parse::<usize>().unwrap(),
            group_separator,
        ),
        (false, false, true) => print_with_before_context(
            &mut reader,
            re,
            flags,
            before_context_number.parse::<usize>().unwrap(),
            group_separator
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_count_matches() {
        let regex_pattern = regex::Regex::new("like").unwrap();
        let input_file_path = "./src/data/pessoa.txt";
        let input_file = File::open(input_file_path).unwrap();
        let reader = BufReader::new(input_file);

        let number_of_matches = count_matches(reader,regex_pattern);
        assert_eq!(number_of_matches, 5);
    }
}

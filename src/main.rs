//! Searches for patterns. Prints to the standard output after successful match.
use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

/// Struct representting the argument flags.
///
/// # Respective flags represented by the fields:
/// ```
/// --count, -c
/// --line-number, -n
/// --color
/// ```
struct Flags {
    count: bool,
    line_number: bool,
    colorize: bool,
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
fn choose_process<T: BufRead + Sized>(reader: T, re: Regex, flags: &Flags) {
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
            (true, true) => println!("{}: {}", i + 1, re.replace_all(&line, colorize_pattern(pattern))),
            _ => println!("{}", line),
        }
    }
}

fn main() {
    let args = App::new("grab")
        .version("0.1")
        .about("Searches for patterns. Prints to the standard output after successful match.")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search in. This is optional. If omitted, takes input from STDIN.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("count")
                .help("Supress normal output and instead print number of matching lines.")
                .short("c")
                .long("count")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("line numbers")
            .help("Prefix each line of output with the 1-based line number within its input file.")
            .short("n")
            .long("line-number")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("color")
            .help("Highlight the matched terms on every line with red color.")
            .long("color")
            .takes_value(false)
            .required(false)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap_or("STDIN");

    let count_flag = args.is_present("count");
    let line_number_flag = args.is_present("line numbers");
    let color_flag = args.is_present("color");

    let flags = Flags {
        count: count_flag,
        line_number: line_number_flag,
        colorize: color_flag,
    };

    let re = Regex::new(pattern).unwrap();

    // if `input` argument was not given then take input from STDIN
    if input == "STDIN" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        choose_process(reader, re, &flags);
    } else {
        let input_file = File::open(input).unwrap();
        let reader = BufReader::new(input_file);
        choose_process(reader, re, &flags);
    }
}

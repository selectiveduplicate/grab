//! Searches for patterns. Prints to the standard output after successful match.
use clap::{App, Arg};
use regex::RegexBuilder;
use std::fs::File;
use std::io;
use std::io::BufReader;

mod lib;

// Regex is needed in both files, so declared it as `pub` in lib.rs
// and brought it into scope here.
// Is this how it should be done?
use crate::lib::{choose_process, Flags};

fn main() {
    let args = App::new("grab")
        .version("0.1")
        .about("Searches for patterns. Prints lines that match those patterns to the standard output.")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search in. This is optional. If omitted, takes input from STDIN")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("count")
                .help("Supress normal output and instead print number of matching lines")
                .short("c")
                .long("count")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("line_number")
            .help("Prefix each line of output with the 1-based line number within its input file")
            .short("n")
            .long("line-number")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("color")
            .help("Highlight the matched terms on every line with red color")
            .long("color")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("ignore_case")
            .help("Ignore case distinctions (uppercase and lowercase) in patterns and input data, so that characters that differ only in case match each other")
            .long("ignore-case")
            .short("i")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("after_context")
            .help("Prints NUM lines of trailing context after the matching lines. Each group of match and its context is separated by a '---' string.")
            .long("after-context")
            .short("A")
            .value_name("NUM")
            .takes_value(true)
            .required(false)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap_or("STDIN");
    let after_context_number = args.value_of("after_context").unwrap_or("NO_CONTEXT");

    let count_flag = args.is_present("count");
    let line_number_flag = args.is_present("line_number");
    let color_flag = args.is_present("color");
    let ignore_case_flag = args.is_present("ignore_case");
    let after_context_flag = args.is_present("after_context");

    let flags = Flags {
        count: count_flag,
        line_number: line_number_flag,
        colorize: color_flag,
        ignore_case: ignore_case_flag,
        after_context: after_context_flag,
    };

    let built_regex = match ignore_case_flag {
        // if -i flag was supplied, set the value for the case insensitive (i) flag
        true => RegexBuilder::new(&pattern).case_insensitive(true).build(),
        // otherwise, build the Regex normally
        _ => RegexBuilder::new(&pattern).build(),
    };

    let re = built_regex.unwrap();

    // if `input` argument was not given then take input from STDIN
    if input == "STDIN" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        choose_process(reader, re, &flags, after_context_number);
    } else {
        let input_file = File::open(input).unwrap();
        let reader = BufReader::new(input_file);
        choose_process(reader, re, &flags, after_context_number);
    }
}

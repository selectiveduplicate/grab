//! Searches for patterns. Prints to the standard output after successful match.
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::BufReader;
use regex::Regex;

mod lib;

// Regex is needed in both files, so declared it as `pub` in lib.rs
// and brought it into scope here.
// Is this how it should be done?
use crate::lib::{choose_process, Flags};

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

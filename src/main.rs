use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

struct Flags {
    count: bool,
    line_number: bool,
    colorize: bool,
}

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

fn colorize_pattern(pattern: &str) -> String {
    pattern.red().to_string()
}

// This function is horrible. Just horrible code...
fn choose_process<T: BufRead + Sized>(reader: T, re: Regex, flags: &Flags) {
    if flags.count {
        println!("{}", count_matches(reader, re));
    } else {
        if flags.line_number && !flags.colorize {
            for (i, line_) in reader.lines().enumerate() {
                let line = line_.unwrap();
                match re.find(&line) {
                    Some(_) => println!("{}: {}", i + 1, line),
                    None => (),
                }
            }
        } else if flags.line_number && flags.colorize {
            for (i, line_) in reader.lines().enumerate() {
                let line = line_.unwrap();
                let pattern = match re.find(&line) {
                    Some(m) => m.as_str(),
                    None => "none",
                };
                if pattern != "none" {
                    let colorized_pattern = colorize_pattern(&pattern);
                    println!("{}: {}", i + 1, re.replace_all(&line, colorized_pattern));
                }
            }
        } else if !flags.line_number && flags.colorize {
            for line_ in reader.lines() {
                let line = line_.unwrap();
                let pattern = match re.find(&line) {
                    Some(m) => m.as_str(),
                    None => "none",
                };
                if pattern != "none" {
                    let colorized_pattern = colorize_pattern(&pattern);
                    println!("{}", re.replace_all(&line, colorized_pattern));
                }
            }
        } else {
            for line_ in reader.lines() {
                let line = line_.unwrap();
                match re.find(&line) {
                    Some(_) => println!("{}", line),
                    None => (),
                }
            }
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
    let input = args.value_of("input").unwrap_or("-");

    let count_flag = args.is_present("count");
    let line_number_flag = args.is_present("line numbers");
    let color_flag = args.is_present("color");

    let flags = Flags {
        count: count_flag,
        line_number: line_number_flag,
        colorize: color_flag,
    };

    let re = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        choose_process(reader, re, &flags);
    } else {
        let input_file = File::open(input).unwrap();
        let reader = BufReader::new(input_file);
        choose_process(reader, re, &flags);
    }
}

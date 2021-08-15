use regex::RegexBuilder;
use std::fs::File;
use std::io;
use std::io::BufReader;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::choose_process;

fn main() {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap_or("STDIN");
    let after_context_number = args.value_of("after_context").unwrap_or("NO_CONTEXT");
    let before_context_number = args.value_of("before_context").unwrap_or("NO_CONTEXT");
    let group_separator = args.value_of("group_separator").unwrap_or("---");

    let flags = Flags::set_flags(&args);

    let built_regex = match flags.ignore_case {
        // if -i flag was supplied, set the value for the case insensitive (i) flag
        true => RegexBuilder::new(pattern).case_insensitive(true).build(),
        // otherwise, build the Regex normally
        _ => RegexBuilder::new(pattern).build(),
    };

    let re = built_regex.unwrap();

    // if `input` argument was not given then take input from STDIN
    if input == "STDIN" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        choose_process(
            reader,
            re,
            &flags,
            after_context_number,
            before_context_number,
            group_separator,
        );
    } else {
        let input_file = File::open(input).unwrap();
        let reader = BufReader::new(input_file);
        choose_process(
            reader,
            re,
            &flags,
            after_context_number,
            before_context_number,
            group_separator,
        );
    }
}

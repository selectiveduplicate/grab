use regex::RegexBuilder;
use std::fs::File;
use std::io;
use std::io::BufReader;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::choose_process;

fn main() -> Result<(), io::Error> {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap_or("STDIN");
    let after_context_number = args.value_of("after_context").unwrap_or("NO_CONTEXT");
    let before_context_number = args.value_of("before_context").unwrap_or("NO_CONTEXT");
    let group_separator = args.value_of("group_separator").unwrap_or("---");
    let context = args.value_of("context").unwrap_or("NO_CONTEXT");

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
        let mut buffer = String::new();
        while stdin.read_line(&mut buffer).is_ok() {
            choose_process(
                buffer.as_bytes(),
                re.clone(),
                &flags,
                after_context_number,
                before_context_number,
                context,
                group_separator,
            )?;
            buffer.clear();
        }
    } else {
        let input_file = match File::open(input) {
            Ok(file) => file,
            // Exit with explicit error for invalid file
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(input_file);
        choose_process(
            reader,
            re,
            &flags,
            after_context_number,
            before_context_number,
            context,
            group_separator,
        )?;
    }
    Ok(())
}

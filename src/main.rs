use lib::error::CliError;
use regex::RegexBuilder;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::choose_process;

fn main() -> Result<(), CliError> {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));
    let after_context_number = args.value_of("after_context");
    let before_context_number = args.value_of("before_context");
    let group_separator = args.value_of("group_separator").unwrap_or("---");
    let context = args.value_of("context");

    let flags = Flags::set_flags(&args);

    let re = match flags.ignore_case {
        // if -i flag was supplied, set the value for the case insensitive (i) flag
        true => RegexBuilder::new(pattern).case_insensitive(true).build()?,
        // otherwise, build the Regex normally
        _ => RegexBuilder::new(pattern).build()?,
    };

    // if `input` argument was not given then take input from STDIN
    if input == Path::new("STDIN") {
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
        let input_file = File::open(input)?;
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

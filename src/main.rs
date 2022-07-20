use std::path::Path;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::prepare_and_choose;

fn main() {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));
    let group_separator = args.value_of("group_separator").unwrap_or("---");
    let context_details: [Option<&str>; 3] = [
        args.value_of("after_context"),
        args.value_of("before_context"),
        args.value_of("context"),
    ];

    let flags = Flags::set_flags(&args);

    if let Err(e) = prepare_and_choose(
        (pattern, flags.ignore_case),
        input,
        &flags,
        context_details,
        group_separator,
    ) {
        fatal!("{e}");
    }
}

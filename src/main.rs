use std::path::Path;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::prepare_and_choose;
use lib::utils::{parse_context_number, ContextKind};

fn main() {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));
    let group_separator = args.value_of("group_separator").unwrap_or("---");

    let flags = Flags::set_flags(&args);

    let context_kind = if args.is_present("after_context") {
        ContextKind::After(parse_context_number(
            args.value_of("after_context").unwrap(),
        ))
    } else if args.is_present("before_context") {
        ContextKind::Before(parse_context_number(
            args.value_of("before_context").unwrap(),
        ))
    } else if args.is_present("context") {
        ContextKind::AfterAndBefore(parse_context_number(args.value_of("context").unwrap()))
    } else {
        ContextKind::None
    };

    if let Err(e) = prepare_and_choose(
        (pattern, flags.ignore_case),
        input,
        &flags,
        context_kind,
        group_separator,
    ) {
        fatal!("{e}");
    }
}

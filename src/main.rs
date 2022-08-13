use std::path::Path;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::prepare_and_choose;
use lib::utils::ContextKind;

fn main() {
    let args = Cli::new().parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));
    let group_separator = args.value_of("group_separator").unwrap_or("---");

    let flags = Flags::set_flags(&args);

    let context_kind = if args.is_present("after_context") {
        ContextKind::After(args.value_of("after_context").unwrap())
    } else if args.is_present("before_context") {
        ContextKind::Before(args.value_of("before_context").unwrap())
    } else if args.is_present("context") {
        ContextKind::AfterAndBefore(args.value_of("context").unwrap())
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
        fatal!("error: {e}");
    }
}

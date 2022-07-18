use std::path::Path;

mod lib;

use lib::cli::Cli;
use lib::flag::Flags;
use lib::process::prepare_and_choose;

/// Writes to the standard error stream and terminates the current process.
#[macro_export]
macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}

fn main() {
    let app = Cli::new();
    let args = app.parse();

    let pattern = args.value_of("pattern").unwrap();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));
    let after_context_number = args.value_of("after_context");
    let before_context_number = args.value_of("before_context");
    let group_separator = args.value_of("group_separator").unwrap_or("---");
    let context = args.value_of("context");

    let flags = Flags::set_flags(&args);

    if let Err(e) = prepare_and_choose(
        (pattern, flags.ignore_case),
        input,
        &flags,
        after_context_number,
        before_context_number,
        context,
        group_separator,
    ) {
        fatal!("{e}");
    }
}

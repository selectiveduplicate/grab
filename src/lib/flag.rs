use clap::ArgMatches;

/// Struct representting the argument flags.
///
/// # Respective flags represented by the fields:
/// ```
/// --count, -c
/// --line-number, -n
/// --color
/// --ignore-case, -i
/// --after-context, -A,
/// --before-context, -B,
/// ```
#[derive(Debug, Default)]
pub struct Flags {
    pub count: bool,
    pub line_number: bool,
    pub colorize: bool,
    pub ignore_case: bool,
    pub after_context: bool,
    pub before_context: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags::default()
    }

    pub fn set_flags(a: &ArgMatches) -> Self {
        let mut flags = Self::new();

        flags.count = a.is_present("count");
        flags.line_number = a.is_present("line_number");
        flags.colorize = a.is_present("color");
        flags.ignore_case = a.is_present("ignore_case");
        flags.after_context = a.is_present("after_context");
        flags.before_context = a.is_present("before_context");

        flags
    }
}

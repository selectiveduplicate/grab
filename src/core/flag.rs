use clap::ArgMatches;

/// Struct representting the argument flags.
///
/// # Respective flags represented by the fields:
/// ```
/// --count, -c
/// --line-number, -n
/// --color
/// --ignore-case, -i
/// --invert-match, -v
/// --after-context, -A,
/// --before-context, -B,
/// --context, -C,
/// ```
#[derive(Debug, Default)]
pub struct Flags {
    pub count: bool,
    pub line_number: bool,
    pub colorize: bool,
    pub ignore_case: bool,
    pub invert_match: bool,
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
        flags.invert_match = a.is_present("invert_match");

        flags
    }
}

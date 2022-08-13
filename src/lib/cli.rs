use clap::{App, Arg, ArgMatches};

///Struct encapsulating the CLI and its arguments.
pub(crate) struct Cli<'cli> {
    app: App<'cli>,
}

impl<'cli> Cli<'cli> {
    ///Returns a new `clap::App` instance.
    pub(crate) fn new() -> Self {
        let app = App::new("grab")
        .version("1.0")
        .author("Abu Sakib <mabusakib@gmail.com>")
        .about("Searches for patterns. Prints lines that match those patterns to the standard output.")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search in. This is optional. If omitted, takes input from STDIN")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("count")
                .help("Supresses normal output and instead prints number of matching lines")
                .short('c')
                .long("count")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("line_number")
            .help("Prefixes each line of output with the 1-based line number within its input file")
            .short('n')
            .long("line-number")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("color")
            .help("Highlights the matched terms on every line with red color")
            .long("color")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("ignore_case")
            .help("Ignores case distinctions (uppercase and lowercase) in patterns and input data, so that characters that differ only in case match each other")
            .long("ignore-case")
            .short('i')
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("invert_match")
            .help("Inverts the sense of matching, to select non-matching lines")
            .long("invert-match")
            .short('v')
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("after_context")
            .help("Prints NUM lines of trailing context after the matching lines. Each group of match and its context is separated by a separator as described by the --group-separator option")
            .long("after-context")
            .short('A')
            .value_name("NUM")
            .takes_value(true)
            .required(false)
        ).arg(
            Arg::with_name("before_context")
            .help("Prints NUM lines of leading context before the matching lines. Each group of match and its context is separated by a separator as described by the --group-separator option")
            .long("before-context")
            .short('B')
            .value_name("NUM")
            .takes_value(true)
            .required(false)
        ).arg(
            Arg::with_name("context")
            .help("Prints NUM lines of context lines before and after the matching lines. Each group of match and its context is separated by a separator as described by the --group-separator option")
            .long("context")
            .short('C')
            .value_name("NUM")
            .takes_value(true)
            .required(false)
        ).arg(
            Arg::with_name("group_separator")
            .help("Use SEP as a group separator. By default SEP is a triple hyphen (---)")
            .long("group-separator")
            .value_name("SEP")
            .takes_value(true)
            .required(false)
        );

        Self { app }
    }

    ///Parses all the command-line arguments.
    pub(crate) fn parse(self) -> ArgMatches {
        self.app.get_matches()
    }
}

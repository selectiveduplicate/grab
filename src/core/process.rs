use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::getwriter;
use crate::core::error::CliError;
use crate::core::flag::Flags;
use crate::core::utils::{compile_regex, parse_context_number, Colors, ContextKind};

/// Calculates the number of matches found
/// according to the regex pattern and returns it.
fn count_matches<T: BufRead + Sized>(reader: T, re: Regex) -> u32 {
    let mut matches: u32 = 0;
    reader.lines().for_each(|line| {
        re.find(&line.unwrap()).is_some().then(|| matches += 1);
    });
    matches
}

/// Prints trailing context lines with or without line numbers.
/// Each group of match and its context is separated by `group_separator`.
fn print_with_after_context<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
    group_separator: &str,
    mut writer: impl Write,
) -> Result<(), CliError> {
    // We need to iterate over the `reader` content twice, which is not possible so
    // we move them to a Vector that we can iterate over more than once.
    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;
    // For line numbers where matches occur
    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    // Stores each matching line and line number as a tuple Vector
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        if re.find(line_).is_some() {
            matched_line_numbers.push(i);
            let v = Vec::with_capacity(context_number + 1);
            matched_lines_with_number.push(v);
        } else {
            continue;
        }
    }

    // We need to iterate `matched_number` of times
    // to find find trailing context lines for each.
    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            let upper_bound = matched_number + context_number;
            if (i >= *matched_number) && (i <= upper_bound) {
                if (i == *matched_number) && (flags.colorize) {
                    let mut matched_line = line.clone();
                    // colorize the matches
                    re.find_iter(line).for_each(|matched| {
                        matched_line = re
                            .replace_all(
                                &matched_line,
                                Colors::colorize_pattern(Colors::Red, matched.as_str()),
                            )
                            .to_string()
                    });
                    matched_lines_with_number[j].push((i, matched_line))
                } else {
                    matched_lines_with_number[j].push((i, line.clone()));
                }
            }
        }
    }

    // Prints matches with context lines.
    // Each group is separated by `group_separator`.
    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        if !is_last && !is_first {
            writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?
        }
        if flags.line_number {
            for (i, line) in matched_line.iter() {
                writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?
            }
        } else {
            for (_, line) in matched_line {
                writeln!(writer, "{}", line)?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}

/// Prints leading context lines with or without line numbers.
/// Each group of match and its context is separated by `group_separator`.
fn print_with_before_context<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
    group_separator: &str,
    mut writer: impl Write,
) -> Result<(), CliError> {
    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;

    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        if re.find(line_).is_some() {
            matched_line_numbers.push(i);
            let v = Vec::with_capacity(context_number + 1);
            matched_lines_with_number.push(v);
        } else {
            continue;
        }
    }

    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            // starting point is always positive
            // this handles the case where a match exists on the first line
            let starting_point = matched_number.saturating_sub(context_number);
            if (i >= starting_point) && (i <= *matched_number) {
                if (i == *matched_number) && (flags.colorize) {
                    let mut matched_line = line.clone();
                    //colorize the matches
                    re.find_iter(line).for_each(|matched| {
                        matched_line = re
                            .replace_all(
                                &matched_line,
                                Colors::colorize_pattern(Colors::Red, matched.as_str()),
                            )
                            .to_string()
                    });
                    matched_lines_with_number[j].push((i, matched_line))
                } else {
                    matched_lines_with_number[j].push((i, line.clone()));
                }
            }
        }
    }

    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        if !is_last && !is_first {
            writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?;
        }
        if flags.line_number {
            for (i, line) in matched_line.iter() {
                writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?;
            }
        } else {
            for (_, line) in matched_line.iter() {
                writeln!(writer, "{}", line)?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}

/// Prints leading and trailing context lines with or without line numbers.
/// Each group of match and its context is separated by `group_separator`.
fn print_with_context<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
    group_separator: &str,
    mut writer: impl Write,
) -> Result<(), CliError> {
    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;

    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        if re.find(line_).is_some() {
            matched_line_numbers.push(i);
            let v = Vec::with_capacity(context_number + 1);
            matched_lines_with_number.push(v);
        } else {
            continue;
        }
    }

    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        // Upper and lower bounds of leading and trailing contxt lines
        // respectively, for each matching line
        let lower_bound = matched_number.saturating_sub(context_number);
        let upper_bound = matched_number + context_number;
        for (i, line) in lines.iter().enumerate() {
            if (i >= lower_bound) && (i <= upper_bound) {
                if (i == *matched_number) && (flags.colorize) {
                    let mut matched_line = line.clone();
                    //colorize the matches
                    re.find_iter(line).for_each(|matched| {
                        matched_line = re
                            .replace_all(
                                &matched_line,
                                Colors::colorize_pattern(Colors::Red, matched.as_str()),
                            )
                            .to_string()
                    });
                    matched_lines_with_number[j].push((i, matched_line))
                } else {
                    matched_lines_with_number[j].push((i, line.clone()));
                }
            }
        }
    }
    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        if !is_last && !is_first {
            writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?;
        }
        if flags.line_number {
            for (i, line) in matched_line.iter() {
                writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?;
            }
        } else {
            for (_, line) in matched_line.iter() {
                writeln!(writer, "{}", line)?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}

/// Checks whether `path` is the standard input stream or a file
/// and calls `choose_process` accordingly.
pub(crate) fn prepare_and_choose(
    needle: (&str, bool),
    path: &std::path::Path,
    flags: &Flags,
    context: ContextKind,
    group_separator: &str,
) -> Result<(), CliError> {
    let re = compile_regex(needle.0, needle.1)?;
    if path == Path::new("STDIN") {
        let stdin = io::stdin();
        let stdin_reader = BufReader::new(stdin.lock());
        let writer = getwriter!();
        choose_process(stdin_reader, re, writer, flags, context, group_separator)?;
    } else {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let writer = getwriter!();
        choose_process(reader, re, writer, flags, context, group_separator)?;
    }
    Ok(())
}

/// Checks the runtime arguments provided by the user and
/// calls the appropriate method.
fn choose_process<T: BufRead + Sized>(
    mut reader: T,
    re: regex::Regex,
    writer: impl Write,
    flags: &Flags,
    context: ContextKind,
    group_separator: &str,
) -> Result<(), CliError> {
    if flags.count {
        println!("{}", count_matches(reader, re));
        return Ok(());
    } else if flags.invert_match {
        print_invert_matches(reader, re, flags, writer)?;
        return Ok(());
    }
    match context {
        ContextKind::After(after_ctx) => {
            let after_ctx = parse_context_number(after_ctx)?;
            print_with_after_context(&mut reader, re, flags, after_ctx, group_separator, writer)?
        }
        ContextKind::Before(before_ctx) => {
            let before_ctx = parse_context_number(before_ctx)?;
            print_with_before_context(&mut reader, re, flags, before_ctx, group_separator, writer)?
        }

        ContextKind::AfterAndBefore(both_ctx) => {
            let both_ctx = parse_context_number(both_ctx)?;
            print_with_context(&mut reader, re, flags, both_ctx, group_separator, writer)?
        }
        ContextKind::None => print_matches(reader, re, flags, writer)?,
    };
    Ok(())
}

/// Prints the lines containing the matches found.
/// Based on the status of the `line_number` field of Flag struct,
/// also prints the 1-based line number preceeding each line.
fn print_matches<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    mut writer: impl Write,
) -> Result<(), CliError> {
    // `.lines()` returns an iterator over each line of `reader`, in the form of `io::Result::String`
    // So a line would be an instance like this: `Ok(line)`
    // `enumerate` gives us the (index, value) pair
    let mut lines = reader.lines().enumerate();

    // `.next()` on an iterator returns the item wrapped in an Option
    // So Each `Some` variant of that option will hold the (index, value) pair
    while let Some((i, Ok(line))) = lines.next() {
        if re.find(&line).is_none() {
            continue;
        }
        let match_iter = re.find_iter(&line);
        let mut matched_line = line.clone();

        match (flags.colorize, flags.line_number) {
            (true, true) => {
                // colorize the patterns
                for mat in match_iter {
                    /* matched_line = format!(
                        "{}",
                        re.replace_all(
                            &matched_line,
                            Colors::colorize_pattern(Colors::Red, mat.as_str())
                        )
                    ); */
                    matched_line = format!(
                        "{}: {}",
                        Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                        re.replace_all(
                            &matched_line,
                            Colors::colorize_pattern(Colors::Red, mat.as_str())
                        )
                    );
                }
                // add colored line numbers
                
            }
            (true, false) => {
                for mat in match_iter {
                    matched_line = format!(
                        "{}",
                        re.replace_all(
                            &matched_line,
                            Colors::colorize_pattern(Colors::Red, mat.as_str())
                        )
                    );
                }
            }
            (false, true) => {
                matched_line = format!(
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    matched_line
                );
            }
            _ => (),
        }

        writeln!(writer, "{}", matched_line)?;
    }
    writer.flush()?;
    Ok(())
}

/// Prints the lines that doesn't contain the pattern.
/// Based on the status of the `line_number` field of Flag struct,
/// also prints the 1-based line number preceeding each line.
fn print_invert_matches<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    mut writer: impl Write,
) -> Result<(), CliError> {
    let mut lines = reader.lines().enumerate();
    while let Some((i, Ok(line))) = lines.next() {
        // don't do anything if match is found
        if re.find(&line).is_some() {
            continue;
        };
        if flags.line_number {
            writeln!(
                writer,
                "{}: {}",
                Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                line
            )?;
        } else {
            writeln!(writer, "{}", line)?;
        }
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::RegexBuilder;

    use super::*;
    use crate::Flags;
    use std::fs::File;
    use std::io::BufReader;

    fn test_inputs(pattern: &str) -> (BufReader<File>, Regex, Vec<u8>) {
        let writer = Vec::new();
        let file = File::open("src/data/pessoa.txt").unwrap();
        let reader = BufReader::new(file);
        let regex = RegexBuilder::new(pattern).build().unwrap();

        (reader, regex, writer)
    }

    #[test]
    fn number_of_matches() {
        let (reader, regex, _) = test_inputs("like");
        let number_of_matches = count_matches(reader, regex);
        assert_eq!(number_of_matches, 5);
    }

    #[test]
    fn print_matches_with_line_number() {
        let flags = Flags {
            count: false,
            line_number: true,
            colorize: false,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_matches(reader, regex, &flags, &mut writer).unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m6\u{1b}[39m: distresses me like a letter of farewell. I feel as if I’m always on the\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn print_matches_without_line_number() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: false,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::None,
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "distresses me like a letter of farewell. I feel as if I’m always on the\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn after_context_without_line_number() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: false,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::After("3"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "distresses me like a letter of farewell. I feel as if I’m always on the
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
reach somewhere. But there’s this heavy slumber that moves from one\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn after_context_with_line_number() {
        let flags = Flags {
            count: false,
            line_number: true,
            colorize: false,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::After("3"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m6\u{1b}[39m: distresses me like a letter of farewell. I feel as if I’m always on the
\u{1b}[32m7\u{1b}[39m: verge of waking up. I’m oppressed by the very self that encases me,
\u{1b}[32m8\u{1b}[39m: asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[32m9\u{1b}[39m: reach somewhere. But there’s this heavy slumber that moves from one\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn after_context_with_ten_char_words_and_color() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs(r"\b\w{10}\b");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::After("2"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "confused landscape, along with \u{1b}[31meverything\u{1b}[39m else.

In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[34m####\u{1b}[39m
\u{1b}[31mdistresses\u{1b}[39m me like a letter of farewell. I feel as if I’m always on the
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[34m####\u{1b}[39m
group of my \u{1b}[31msensations\u{1b}[39m to another, like drifting clouds that make the
half-shaded grass of sprawling fields turn various colours of sun and
green.
\u{1b}[34m####\u{1b}[39m
illusions or moments of calm, large hopes \u{1b}[31mchannelled\u{1b}[39m into the
landscape, sorrows like closed rooms, certain voices, a huge weariness,
the unwritten gospel.
\u{1b}[34m####\u{1b}[39m
We all have our vanity, and that vanity is our way of \u{1b}[31mforgetting\u{1b}[39m that
there are other people with a soul like our own. My vanity consists of\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn before_context_without_line_number() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::Before("3"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "confused landscape, along with everything else.

In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[31mdistress\u{1b}[39mes me like a letter of farewell. I feel as if I’m always on the\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn before_context_with_line_number() {
        let flags = Flags {
            count: false,
            line_number: true,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::Before("3"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m3\u{1b}[39m: confused landscape, along with everything else.
\u{1b}[32m4\u{1b}[39m: 
\u{1b}[32m5\u{1b}[39m: In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[32m6\u{1b}[39m: \u{1b}[31mdistress\u{1b}[39mes me like a letter of farewell. I feel as if I’m always on the\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn context_with_line_number() {
        let flags = Flags {
            count: false,
            line_number: true,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("like");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::AfterAndBefore("2"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m4\u{1b}[39m: 
\u{1b}[32m5\u{1b}[39m: In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[32m6\u{1b}[39m: distresses me \u{1b}[31mlike\u{1b}[39m a letter of farewell. I feel as if I’m always on the
\u{1b}[32m7\u{1b}[39m: verge of waking up. I’m oppressed by the very self that encases me,
\u{1b}[32m8\u{1b}[39m: asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[34m####\u{1b}[39m
\u{1b}[32m8\u{1b}[39m: asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[32m9\u{1b}[39m: reach somewhere. But there’s this heavy slumber that moves from one
\u{1b}[32m10\u{1b}[39m: group of my sensations to another, \u{1b}[31mlike\u{1b}[39m drifting clouds that make the
\u{1b}[32m11\u{1b}[39m: half-shaded grass of sprawling fields turn various colours of sun and
\u{1b}[32m12\u{1b}[39m: green.
\u{1b}[34m####\u{1b}[39m
\u{1b}[32m12\u{1b}[39m: green.
\u{1b}[32m13\u{1b}[39m: 
\u{1b}[32m14\u{1b}[39m: I’m \u{1b}[31mlike\u{1b}[39m someone searching at random, not knowing what object he’s
\u{1b}[32m15\u{1b}[39m: looking for nor where it was hidden. We play hide-and-seek with no
\u{1b}[32m16\u{1b}[39m: one. There’s a transcendent trick in all of this, a fluid divinity we can
\u{1b}[34m####\u{1b}[39m
\u{1b}[32m19\u{1b}[39m: Yes, I reread these pages that represent worthless hours, brief
\u{1b}[32m20\u{1b}[39m: illusions or moments of calm, large hopes channelled into the
\u{1b}[32m21\u{1b}[39m: landscape, sorrows \u{1b}[31mlike\u{1b}[39m closed rooms, certain voices, a huge weariness,
\u{1b}[32m22\u{1b}[39m: the unwritten gospel.
\u{1b}[32m23\u{1b}[39m: 
\u{1b}[34m####\u{1b}[39m
\u{1b}[32m23\u{1b}[39m: 
\u{1b}[32m24\u{1b}[39m: We all have our vanity, and that vanity is our way of forgetting that
\u{1b}[32m25\u{1b}[39m: there are other people with a soul \u{1b}[31mlike\u{1b}[39m our own. My vanity consists of\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn context_without_line_number() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs("like");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::AfterAndBefore("2"),
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "
In these times when an abyss opens up in my soul, the tiniest detail
distresses me \u{1b}[31mlike\u{1b}[39m a letter of farewell. I feel as if I’m always on the
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[34m####\u{1b}[39m
asphyxiated by conclusions, and I’d gladly scream if my voice could
reach somewhere. But there’s this heavy slumber that moves from one
group of my sensations to another, \u{1b}[31mlike\u{1b}[39m drifting clouds that make the
half-shaded grass of sprawling fields turn various colours of sun and
green.
\u{1b}[34m####\u{1b}[39m
green.

I’m \u{1b}[31mlike\u{1b}[39m someone searching at random, not knowing what object he’s
looking for nor where it was hidden. We play hide-and-seek with no
one. There’s a transcendent trick in all of this, a fluid divinity we can
\u{1b}[34m####\u{1b}[39m
Yes, I reread these pages that represent worthless hours, brief
illusions or moments of calm, large hopes channelled into the
landscape, sorrows \u{1b}[31mlike\u{1b}[39m closed rooms, certain voices, a huge weariness,
the unwritten gospel.

\u{1b}[34m####\u{1b}[39m

We all have our vanity, and that vanity is our way of forgetting that
there are other people with a soul \u{1b}[31mlike\u{1b}[39m our own. My vanity consists of\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn invert_matches_without_line_number() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: false,
            ignore_case: false,
            invert_match: true,
        };
        let (reader, regex, mut writer) = test_inputs("like");
        print_invert_matches(reader, regex, &flags, &mut writer).unwrap();
        assert_eq!(
            writer,
            "Like someone on a hill who tries to make out the people in the
valley, I look down at myself from on high, and I’m a hazy and
confused landscape, along with everything else.

In these times when an abyss opens up in my soul, the tiniest detail
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
reach somewhere. But there’s this heavy slumber that moves from one
half-shaded grass of sprawling fields turn various colours of sun and
green.

looking for nor where it was hidden. We play hide-and-seek with no
one. There’s a transcendent trick in all of this, a fluid divinity we can
only hear.

Yes, I reread these pages that represent worthless hours, brief
illusions or moments of calm, large hopes channelled into the
the unwritten gospel.

We all have our vanity, and that vanity is our way of forgetting that\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn invert_matches_with_line_number() {
        let flags = Flags {
            count: false,
            line_number: true,
            colorize: false,
            ignore_case: false,
            invert_match: true,
        };
        let (reader, regex, mut writer) = test_inputs("like");
        print_invert_matches(reader, regex, &flags, &mut writer).unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m1\u{1b}[39m: Like someone on a hill who tries to make out the people in the
\u{1b}[32m2\u{1b}[39m: valley, I look down at myself from on high, and I’m a hazy and
\u{1b}[32m3\u{1b}[39m: confused landscape, along with everything else.
\u{1b}[32m4\u{1b}[39m: 
\u{1b}[32m5\u{1b}[39m: In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[32m7\u{1b}[39m: verge of waking up. I’m oppressed by the very self that encases me,
\u{1b}[32m8\u{1b}[39m: asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[32m9\u{1b}[39m: reach somewhere. But there’s this heavy slumber that moves from one
\u{1b}[32m11\u{1b}[39m: half-shaded grass of sprawling fields turn various colours of sun and
\u{1b}[32m12\u{1b}[39m: green.
\u{1b}[32m13\u{1b}[39m: 
\u{1b}[32m15\u{1b}[39m: looking for nor where it was hidden. We play hide-and-seek with no
\u{1b}[32m16\u{1b}[39m: one. There’s a transcendent trick in all of this, a fluid divinity we can
\u{1b}[32m17\u{1b}[39m: only hear.
\u{1b}[32m18\u{1b}[39m: 
\u{1b}[32m19\u{1b}[39m: Yes, I reread these pages that represent worthless hours, brief
\u{1b}[32m20\u{1b}[39m: illusions or moments of calm, large hopes channelled into the
\u{1b}[32m22\u{1b}[39m: the unwritten gospel.
\u{1b}[32m23\u{1b}[39m: 
\u{1b}[32m24\u{1b}[39m: We all have our vanity, and that vanity is our way of forgetting that\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn multiple_matches_in_same_line_with_color() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: true,
            ignore_case: false,
            invert_match: false,
        };
        let (reader, regex, mut writer) = test_inputs(r"\bour\b");
        choose_process(
            reader,
            regex,
            &mut writer,
            &flags,
            ContextKind::None,
            "####",
        )
        .unwrap();
        assert_eq!(
            writer,
            "We all have \u{1b}[31mour\u{1b}[39m vanity, and that vanity is \u{1b}[31mour\u{1b}[39m way of forgetting that
there are other people with a soul like \u{1b}[31mour\u{1b}[39m own. My vanity consists of\n"
                .as_bytes()
                .to_vec()
        );
    }
}

use regex::Regex;
use std::io::{BufRead, Write};

use crate::lib::flag::Flags;
use crate::lib::utils::Colors;

/// Calculates the number of matches found according to the regex pattern and returns it
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
) -> Result<(), std::io::Error> {
    // For all the matched words/terms
    let mut patterns: Vec<String> = Vec::new();

    // We need to iterate over the `reader` content twice, which is not possible so
    // we move them to a Vector that we can iterate over more than once.
    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;
    // For line numbers where matches occur
    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    // Stores each matching line and line number as a tuple Vector
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        match re.find(line_) {
            Some(pattern) => patterns.push(pattern.as_str().to_string()),
            None => continue,
        }

        matched_line_numbers.push(i);
        let v = Vec::with_capacity(context_number + 1);
        matched_lines_with_number.push(v);
    }

    // We need to iterate `matched_number` of times
    // to find find trailing context lines for each.
    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            let upper_bound = matched_number + context_number;
            if (i >= *matched_number) && (i <= upper_bound) {
                match ((i == *matched_number), flags.colorize) {
                    (true, true) => {
                        let match_iter = re.find_iter(line);
                        let mut matched_line = line.clone();
                        for mat in match_iter {
                            matched_line = re
                                .replace(
                                    &matched_line,
                                    Colors::colorize_pattern(Colors::Red, mat.as_str()),
                                )
                                .to_string();
                        }
                        matched_lines_with_number[j].push((i, matched_line))
                    }
                    (true, _) => matched_lines_with_number[j].push((i, line.clone())),
                    _ => matched_lines_with_number[j].push((i, line.clone())),
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
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?,
        }
        for (i, line) in matched_line.iter() {
            match flags.line_number {
                true => writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?,
                _ => writeln!(writer, "{}", line)?,
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
) -> Result<(), std::io::Error> {
    let mut patterns: Vec<String> = Vec::new();

    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;

    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        match re.find(line_) {
            Some(pattern) => patterns.push(pattern.as_str().to_string()),
            None => continue,
        }

        matched_line_numbers.push(i);
        let v = Vec::with_capacity(context_number + 1);
        matched_lines_with_number.push(v);
    }

    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            // starting point is always positive
            // this handles the case where a match exists on the first line
            let starting_point = matched_number.saturating_sub(context_number);
            if (i >= starting_point) && (i <= *matched_number) {
                match ((i == *matched_number), flags.colorize) {
                    (true, true) => {
                        let match_iter = re.find_iter(line);
                        let mut matched_line = line.clone();
                        for mat in match_iter {
                            matched_line = re
                                .replace(
                                    &matched_line,
                                    Colors::colorize_pattern(Colors::Red, mat.as_str()),
                                )
                                .to_string();
                        }
                        matched_lines_with_number[j].push((i, matched_line))
                    }
                    _ => matched_lines_with_number[j].push((i, line.clone())),
                }
            }
        }
    }

    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?,
        }
        for (i, line) in matched_line.iter() {
            match flags.line_number {
                true => writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?,
                _ => writeln!(writer, "{}", line)?,
            }
        }
    }
    writer.flush()?;
    Ok(())
}

/// Prints leading and trailing context lines with or without line numbers.
/// Each group of match and its context is separated by `group_separator`.
fn print_with_context<T: BufRead + Sized>(
    reader: &mut T,
    re: Regex,
    flags: &Flags,
    context_number: usize,
    group_separator: &str,
) -> Result<(), std::io::Error> {
    let mut patterns: Vec<String> = Vec::new();

    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;

    let mut matched_line_numbers: Vec<usize> = Vec::with_capacity(lines.len());
    let mut matched_lines_with_number: Vec<Vec<(usize, String)>> = Vec::with_capacity(lines.len());

    for (i, line_) in lines.iter().enumerate() {
        match re.find(line_) {
            Some(pattern) => patterns.push(pattern.as_str().to_string()),
            None => continue,
        }

        matched_line_numbers.push(i);
        let v = Vec::with_capacity(context_number + 1);
        matched_lines_with_number.push(v);
    }

    for (j, matched_number) in matched_line_numbers.iter().enumerate() {
        for (i, line) in lines.iter().enumerate() {
            // Upper and lower bounds of leading and trailing contxt lines
            // respectively, for each matching line
            let lower_bound = matched_number.saturating_sub(context_number);
            let upper_bound = matched_number + context_number;
            if (i >= lower_bound) && (i <= upper_bound) {
                match ((i == *matched_number), flags.colorize) {
                    (true, true) => {
                        let match_iter = re.find_iter(line);
                        let mut matched_line = line.clone();
                        for mat in match_iter {
                            matched_line = re
                                .replace(
                                    &matched_line,
                                    Colors::colorize_pattern(Colors::Red, mat.as_str()),
                                )
                                .to_string();
                        }
                        matched_lines_with_number[j].push((i, matched_line))
                    }
                    _ => matched_lines_with_number[j].push((i, line.clone())),
                }
            }
        }
    }

    // Prepare stdout for printing to it
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut writer = std::io::BufWriter::new(handle);

    for (matched_line, is_last, is_first) in matched_lines_with_number
        .iter()
        .enumerate()
        .map(|(index, m)| (m, index == matched_lines_with_number.len(), index == 0))
    {
        match (is_last, is_first) {
            (true, _) => (),
            (_, true) => (),
            _ => writeln!(
                writer,
                "{}",
                Colors::colorize_pattern(Colors::Blue, group_separator)
            )?,
        }
        for (i, line) in matched_line.iter() {
            match flags.line_number {
                true => writeln!(
                    writer,
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    line
                )?,
                _ => writeln!(writer, "{}", line)?,
            }
        }
    }
    writer.flush()?;
    Ok(())
}

/// Checks status of the `count` field in Flags struct.
/// If it's set, then display number of matches returned by calling `count_matches`.
/// Otherwise, call `print_matches`.
pub fn choose_process<T: BufRead + Sized>(
    mut reader: T,
    re: Regex,
    flags: &Flags,
    after_context_number: Option<&str>,
    before_context_number: Option<&str>,
    context: Option<&str>,
    group_separator: &str,
) -> Result<(), std::io::Error> {
    match (
        flags.count,
        flags.after_context,
        flags.before_context,
        flags.context,
        flags.invert_match,
    ) {
        (true, _, _, _, _) => {
            println!("{}", count_matches(reader, re));
            Ok(())
        }
        (false, false, false, false, false) => {
            let stdout = std::io::stdout();
            let handle = stdout.lock();
            let writer = std::io::BufWriter::new(handle);
            print_matches(reader, re, flags, writer)?;
            Ok(())
        }
        (false, true, _, _, _) => match after_context_number.unwrap().parse::<usize>() {
            Ok(context) => {
                let stdout = std::io::stdout();
                let handle = stdout.lock();
                let writer = std::io::BufWriter::new(handle);
                print_with_after_context(&mut reader, re, flags, context, group_separator, writer)?;
                Ok(())
            }
            // Exit with explicit error if context length isn't a valid integer
            Err(_) => {
                eprintln!("Invalid context length argument: must be an integer.");
                std::process::exit(1);
            }
        },
        (false, false, true, _, _) => {
            match before_context_number.unwrap().parse::<usize>() {
                Ok(context) => {
                    let stdout = std::io::stdout();
                    let handle = stdout.lock();
                    let writer = std::io::BufWriter::new(handle);
                    print_with_before_context(&mut reader, re, flags, context, group_separator, writer)?;
                    Ok(())
                }
                // Exit with explicit error if context length isn't a valid integer
                Err(_) => {
                    eprintln!("Invalid context length argument: must be an integer.");
                    std::process::exit(1);
                }
            }
        }
        (_, _, _, true, _) => {
            match context.unwrap().parse::<usize>() {
                Ok(context) => print_with_context(&mut reader, re, flags, context, group_separator),
                // Exit with explicit error if context length isn't a valid integer
                Err(_) => {
                    eprintln!("Invalid context length argument: must be an integer.");
                    std::process::exit(1);
                }
            }
        }
        (_, _, _, _, true) => print_invert_matches(reader, re, flags),
    }
}

/// Prints the lines containing the matches found.
/// Based on the status of the `line_number` field of Flag struct,
/// also prints the 1-based line number preceeding each line.
fn print_matches<T: BufRead + Sized>(
    reader: T,
    re: Regex,
    flags: &Flags,
    mut writer: impl Write,
) -> Result<(), std::io::Error> {
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
                    matched_line = format!(
                        "{}",
                        re.replace(
                            &matched_line,
                            Colors::colorize_pattern(Colors::Red, mat.as_str())
                        )
                    );
                }
                // add colored line numbers
                matched_line = format!(
                    "{}: {}",
                    Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                    matched_line
                );
            }
            (true, false) => {
                for mat in match_iter {
                    matched_line = format!(
                        "{}",
                        re.replace(
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
) -> Result<(), std::io::Error> {
    let mut lines = reader.lines().enumerate();
    // Prepare stdout for printing to it
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut writer = std::io::BufWriter::new(handle);
    while let Some((i, Ok(line))) = lines.next() {
        // don't do anything if match is found
        if re.find(&line).is_some() {
            continue;
        };
        match flags.line_number {
            true => writeln!(
                writer,
                "{}: {}",
                Colors::colorize_pattern(Colors::Green, &format!("{}", i + 1)),
                line
            )?,
            _ => writeln!(writer, "{}", line)?,
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
            after_context: false,
            before_context: false,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_matches(reader, regex, &flags, &mut writer).unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m6\u{1b}[0m: distresses me like a letter of farewell. I feel as if I’m always on the\n"
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
            after_context: false,
            before_context: false,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_matches(reader, regex, &flags, &mut writer).unwrap();
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
            after_context: true,
            before_context: false,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_with_after_context(reader, regex, &flags, 3, "####", &mut writer).unwrap();
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
            after_context: true,
            before_context: false,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_with_after_context(reader, regex, &flags, 3, "####", &mut writer).unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m6\u{1b}[0m: distresses me like a letter of farewell. I feel as if I’m always on the
\u{1b}[32m7\u{1b}[0m: verge of waking up. I’m oppressed by the very self that encases me,
\u{1b}[32m8\u{1b}[0m: asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[32m9\u{1b}[0m: reach somewhere. But there’s this heavy slumber that moves from one\n"
                .as_bytes()
                .to_vec()
        );
    }

    #[test]
    fn after_context_with_for_ten_char_words_and_pattern_color() {
        let flags = Flags {
            count: false,
            line_number: false,
            colorize: true,
            ignore_case: false,
            invert_match: false,
            after_context: true,
            before_context: false,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs(r"\b\w{10}\b");
        print_with_after_context(reader, regex, &flags, 2, "####", &mut writer).unwrap();
        assert_eq!(
            writer,
            "confused landscape, along with \u{1b}[31meverything\u{1b}[0m else.

In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[34m####\u{1b}[0m
\u{1b}[31mdistresses\u{1b}[0m me like a letter of farewell. I feel as if I’m always on the
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
\u{1b}[34m####\u{1b}[0m
group of my \u{1b}[31msensations\u{1b}[0m to another, like drifting clouds that make the
half-shaded grass of sprawling fields turn various colours of sun and
green.
\u{1b}[34m####\u{1b}[0m
illusions or moments of calm, large hopes \u{1b}[31mchannelled\u{1b}[0m into the
landscape, sorrows like closed rooms, certain voices, a huge weariness,
the unwritten gospel.
\u{1b}[34m####\u{1b}[0m
We all have our vanity, and that vanity is our way of \u{1b}[31mforgetting\u{1b}[0m that
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
            after_context: false,
            before_context: true,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_with_before_context(reader, regex, &flags, 3, "####", &mut writer).unwrap();
        assert_eq!(
            writer,
            "confused landscape, along with everything else.

In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[31mdistress\u{1b}[0mes me like a letter of farewell. I feel as if I’m always on the\n"
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
            after_context: false,
            before_context: true,
            context: false,
        };
        let (reader, regex, mut writer) = test_inputs("distress");
        print_with_before_context(reader, regex, &flags, 3, "####", &mut writer).unwrap();
        assert_eq!(
            writer,
            "\u{1b}[32m3\u{1b}[0m: confused landscape, along with everything else.
\u{1b}[32m4\u{1b}[0m: 
\u{1b}[32m5\u{1b}[0m: In these times when an abyss opens up in my soul, the tiniest detail
\u{1b}[32m6\u{1b}[0m: \u{1b}[31mdistress\u{1b}[0mes me like a letter of farewell. I feel as if I’m always on the\n"
                .as_bytes()
                .to_vec()
        );
    }
}

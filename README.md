An attempt at making a simple clone of [grep(1)](https://www.man7.org/linux/man-pages/man1/grep.1.html) using Rust.

It builds up on the first project `grep-lite` presented in the book [Rust in Action](https://www.manning.com/books/rust-in-action) by Tim McNamara.

# Usage
```
USAGE:
    grab [FLAGS] <pattern> [input]

FLAGS:
        --color          Highlight the matched terms on every line with red color.
    -c, --count          Supress normal output and instead print number of matching lines.
    -h, --help           Prints help information
    -n, --line-number    Prefix each line of output with the 1-based line number within its input file.
    -V, --version        Prints version information

ARGS:
    <pattern>    The pattern to search for
    <input>      File to search in. This is optional. If omitted, takes input from STDIN.

```

An attempt at making a simple clone of [grep(1)](https://www.man7.org/linux/man-pages/man1/grep.1.html) using Rust.

It builds up on the first project `grep-lite` presented in the book [Rust in Action](https://www.manning.com/books/rust-in-action) by Tim McNamara.

# Installation
If you don't have Rust installed, follow [the official instructions](https://www.rust-lang.org/tools/install) to install Rust. Then:

* Clone the repo.
* Go to the project directory.
* Run `cargo install --path .` from your terminal. It'll install the binary in your `$HOME/.cargo/bin` directory.
* Alternatively, you can run `cargo build --release` or `cargo build` to just build the project.

# Usage
To see available usages, supply the  `--help` flag to your `grab` executable:

```shell
grab --help
```

# Features
 - [x] `STDIN` mode
 - [x] Colored matches
 - [x] Number of matches
 - [x] Invert matching
 - [x] Case insensitive mode
 - [x] Line numbers
 - [x] Context lines
	 - [x] Trailing context
	 - [x] Leading context
	 - [x] Both trailing and leading context
- [x]  Custom group separator
	- [x] Colored separator


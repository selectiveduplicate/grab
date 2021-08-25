An attempt at making a simple clone of [grep(1)](https://www.man7.org/linux/man-pages/man1/grep.1.html) using Rust.

It builds up on the first project `grep-lite` presented in the book [Rust in Action](https://www.manning.com/books/rust-in-action) by Tim McNamara.

# Installation
* Clone the repo.
* Do a `cargo install` from the root of the project. It'll install the binary in your `$HOME/.cargo/bin` directory.

# Usage
To see available usages, run `grab --help`.

# Features
 - [x] `STDIN` mode
 - [x] Colored matches
 - [x] Number of matches
 - [x] Case insensitive mode
 - [x] Line numbers
 - [x] Context lines
	 - [x] Trailing context
	 - [x] Leading context
- [x]  Custom group separator
	- [x] Colored separator

# Wishlist
- [ ] `--context` flag support like `grep`.

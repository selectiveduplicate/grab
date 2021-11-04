An attempt at making a simple clone of [grep(1)](https://www.man7.org/linux/man-pages/man1/grep.1.html) using Rust.

It builds up on the [lightweight grep-like program](https://github.com/rust-in-action/code/blob/7d7955e9605ca156f6eb7cb5bc9f124c97927d25/ch2/ch2-with-regex.rs) presented in the book [Rust in Action](https://www.manning.com/books/rust-in-action) by Tim McNamara.

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

## Usage examples
* The following searches for the words `Like` or `like`, using the `--ignore-case` flag:

```shell
grab "like" --ignore-case src/data/pessoa.txt
```

It'll output the following text (you can also specify the `--color` flag to colorize the matches):

```
Like someone on a hill who tries to make out the people in the
distresses me like a letter of farewell. I feel as if I’m always on the
group of my sensations to another, like drifting clouds that make the
I’m like someone searching at random, not knowing what object he’s
landscape, sorrows like closed rooms, certain voices, a huge weariness,
there are other people with a soul like our own. My vanity consists of
```

* You can also provide regular expressions instead of just words. For example, the following outputs every line containing 2-letter words:

```shell
grab --color "\b[a-zA-Z]{2}\b" src/data/pessoa.txt
```

* You can print context lines by using the following options, followed by the number of context lines you want:
	* `--after-context`, `-A`
	* `--before-context`, `-B`
	* `--context`, `-C`

For example, the following prints two context lines before and after each matching line:

```shell
grab "like" -C 2 src/data/pessoa.txt
```
This yields the following, with the default separator `---` between each group of match and context:

```
In these times when an abyss opens up in my soul, the tiniest detail
distresses me like a letter of farewell. I feel as if I’m always on the
verge of waking up. I’m oppressed by the very self that encases me,
asphyxiated by conclusions, and I’d gladly scream if my voice could
---
asphyxiated by conclusions, and I’d gladly scream if my voice could
reach somewhere. But there’s this heavy slumber that moves from one
group of my sensations to another, like drifting clouds that make the
half-shaded grass of sprawling fields turn various colours of sun and
green.
---
green.

I’m like someone searching at random, not knowing what object he’s
looking for nor where it was hidden. We play hide-and-seek with no
one. There’s a transcendent trick in all of this, a fluid divinity we can
---
Yes, I reread these pages that represent worthless hours, brief
illusions or moments of calm, large hopes channelled into the
landscape, sorrows like closed rooms, certain voices, a huge weariness,
the unwritten gospel.

---
```
You can use a custom separator by using the `--group-separator` option.

You might've already noticed a difference between `grep` and `grab` for context lines. `grep` will never show you a line more than once, `grab` does.

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


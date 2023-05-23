# txt-sh

`txt-sh` is a Rust-based command-line utility that reads input from the
standard input (stdin) and searches for any occurrences of the pattern `$()`.
Inside the parentheses, it expects a shell command. For each occurrence of the
pattern, the Rust binary executes the shell command inside the parentheses
using the "sh" shell. It then replaces the pattern with the output of the
executed command and prints the final output to the standard output (stdout).

This utility is primarily useful for text processing and templating. You can
use it to process text files or templates containing shell commands, replacing
them with their respective outputs. This can be helpful for generating
configuration files, reports, or other text-based documents that require
dynamic content based on the output of shell commands.

## Usage

To use `txt-sh`, simply pipe the input text or file to the utility:

```bash
$ echo "Today is $(date)" | txt-sh
Today is Mon May 22 22:27:01 EDT 2023
```

Or, for processing a file:

```bash
$ cat input.txt | txt-sh > output.txt
```

The input text or file should contain shell commands wrapped in `$()` that you
want to be replaced with their respective outputs.

For example, if the input file `input.txt` contains the following content:

```
Hello, today is $(date).
The current directory contains $(ls | wc -l) files.
```

After running `txt-sh`, the output will be:

```
Hello, today is Mon Sep 6 12:34:56 PDT 2021.
The current directory contains 42 files.
```


## Installation

To install `txt-sh`, you need to have Rust and Cargo installed on your system.
You can download Rust from the [official
website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/txt-sh.git
cd txt-sh
cargo build --release
```

The compiled binary will be available in the `target/release` directory. You
can move it to a directory in your `PATH` for easy access:

```bash
sudo mv target/release/txt-sh ~/bin
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please
open an issue on the GitHub repository. If you'd like to contribute code, feel
free to fork the repository and submit a pull request.

## License

`txt-sh` is released under the [MIT License](LICENSE).

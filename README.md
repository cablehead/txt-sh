# txt-sh

`txt-sh` is a versatile command-line tool made with Rust that processes text by
executing shell commands within specified patterns and replacing them with the
command's output. This utility is useful for text processing and templating,
allowing you to create dynamic content in text files using shell command
outputs.

## Usage

To use `txt-sh`, pipe the input text:

```bash
$ echo "Today is $(date)" | txt-sh
Today is Mon May 22 22:27:01 EDT 2023
```

Or, for processing a file:

```bash
$ cat input.txt | txt-sh > output.txt
```

You can also provide a file as an argument:

```bash
$ txt-sh input.txt > output.txt
```

The input text or file should contain shell commands wrapped in `$()` that you
want to be replaced with their respective outputs. For piping input to a
command and replacing the pattern with the command's output, use the `>()`
pattern.

For example, the input file `examples/input.txt` has the following content:

```
Hello, today is $(date).
The current directory contains $(ls | wc -l) files.

Here's an additional note: >(cat)
```

```
$ echo Have a good one. | txt-sh examples/input.txt
Hello, today is Tue May 23 09:27:42 EDT 2023.
The current directory contains 9 files.

Here's an additional note: Have a good one.
```

## Installation

To install `txt-sh`, you need to have Rust and Cargo installed on your system. You can download Rust from the [official website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/txt-sh.git
cd txt-sh
cargo build --release
```

The compiled binary will be available in the `target/release` directory. You can move it to a directory in your `PATH` for easy access:

```bash
sudo mv target/release/txt-sh ~/bin
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. If you'd like to contribute code, feel free to fork the repository and submit a pull request.

## License

`txt-sh` is released under the [MIT License](LICENSE).

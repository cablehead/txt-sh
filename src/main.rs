use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    file: Option<PathBuf>,
}

struct CommandOutput {
    stdout: String,
    stderr: Option<String>,
    code: Option<i32>,
}

fn execute_child_command(command: &str, input: Option<&str>) -> CommandOutput {
    let mut child = Command::new("sh");
    child
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if input.is_some() {
        child.stdin(Stdio::piped());
    }

    let mut child = child.spawn().expect("Failed to spawn child process");

    if let Some(input) = input {
        child
            .stdin
            .as_mut()
            .expect("Failed to open stdin")
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    let stdout = String::from_utf8(output.stdout).unwrap().trim().to_string();
    let stderr = if output.stderr.is_empty() {
        None
    } else {
        Some(String::from_utf8(output.stderr).unwrap().trim().to_string())
    };
    let code = if output.status.success() {
        None
    } else {
        Some(output.status.code().unwrap())
    };

    CommandOutput {
        stdout,
        stderr,
        code,
    }
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input from stdin");

    let mut template = String::new();
    match args.file {
        Some(file_path) => {
            let mut file = File::open(file_path).expect("Failed to open file");
            file.read_to_string(&mut template)
                .expect("Failed to read input from file");
        }
        None => {
            template = input.to_string();
        }
    }

    let re = regex::Regex::new(r"(\$\(|>\()(.*?)\)").unwrap();
    let output = re.replace_all(&template, |caps: &regex::Captures<'_>| {
        let command_type = caps.get(1).unwrap().as_str();
        let command = caps.get(2).unwrap().as_str();

        let output = execute_child_command(
            command,
            if command_type == "$(" {
                None
            } else {
                Some(&input)
            },
        );

        if output.stderr.is_some() || output.code.is_some() {
            eprintln!("Running: {}", command);

            if let Some(ref stderr_output) = output.stderr {
                eprintln!("\n```stderr\n{}\n```\n", stderr_output);
            }

            if let Some(code) = output.code {
                eprintln!("exit-code: {}", code);
                std::process::exit(code);
            }
        }

        output.stdout
    });

    println!("{}", output);
}

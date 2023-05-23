use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read};
use std::process::Command;
use std::process::Stdio;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    match args.file {
        Some(file_path) => {
            let mut file = File::open(file_path).expect("Failed to open file");
            file.read_to_string(&mut input)
                .expect("Failed to read input from file");
        }
        None => {
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read input from stdin");
        }
    }

    let re = regex::Regex::new(r"\$\((.*?)\)").unwrap();
    let pipe_re = regex::Regex::new(r">\((.*?)\)").unwrap();
    let output = re.replace_all(&input, |caps: &regex::Captures<'_>| {
        let command = caps.get(1).unwrap().as_str();
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    });

    let output = pipe_re.replace_all(&output, |caps: &regex::Captures<'_>| {
        let command = caps.get(1).unwrap().as_str();
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        io::copy(&mut io::stdin(), child.stdin.as_mut().expect("Failed to open stdin"))
            .expect("Failed to write to stdin");

        let output = child
            .wait_with_output()
            .expect("Failed to wait on child process");

        String::from_utf8(output.stdout).unwrap().trim().to_string()
    });

    println!("{}", output);
}

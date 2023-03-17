use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let re = regex::Regex::new(r"\$\((.*?)\)").unwrap();
    let output = re.replace_all(&input, |caps: &regex::Captures<'_>| {
        let command = caps.get(1).unwrap().as_str();
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    });

    println!("{}", output);
}

use std::io::Write;
use std::process::{Command, Stdio};

// Taken from:
// https://github.com/assert-rs/assert_cmd/blob/e71a9f7b15596dd2aeea911bedbbd1859d84fa67/src/cargo.rs#L183-L208
fn target_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

fn cargo_bin(name: &str) -> std::path::PathBuf {
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    std::env::var_os(&env_var)
        .map(|p| p.into())
        .unwrap_or_else(|| target_dir().join(format!("{}{}", name, std::env::consts::EXE_SUFFIX)))
}

#[test]
fn from_stdin() {
    let txt_sh = cargo_bin("txt-sh");

    let mut child = Command::new(txt_sh)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(b"Hello, $(echo world)!")
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n");
}

#[test]
fn from_file() {
    let txt_sh = cargo_bin("txt-sh");

    // Create a temporary file with the template content
    let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    write!(temp_file, "Hello, $(echo world)!").expect("Failed to write to temporary file");

    // Run the txt-sh command with the temporary file as input
    let output = Command::new(txt_sh)
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n");
}

#[test]
fn from_file_with_pipe() {
    let txt_sh = cargo_bin("txt-sh");

    // Create a temporary file with the template content
    let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    write!(temp_file, "Hello, $(echo world)!\n\n>(cat)")
        .expect("Failed to write to temporary file");

    // Run the txt-sh command with the temporary file as input and pipe the string "How are you?" to it
    let mut child = Command::new(txt_sh)
        .arg(temp_file.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(b"How are you?")
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n\nHow are you?\n");
}

#[test]
fn test_non_zero_exit_status() {
    let txt_sh = cargo_bin("txt-sh");

    let input = "$(echo 1)\n$(eho 2)\n$(echo 3)\n";

    let mut child = Command::new(txt_sh)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr,
        "Running: eho 2\n\n```stderr\nsh: eho: command not found\n```\n\nexit-code: 127\n"
    );
}

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

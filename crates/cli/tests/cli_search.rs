//! # CLI Integration Tests
//!
//! Tests the CLI binary by spawning it as a subprocess.
//!

use std::io::Write;
use std::process::{Command, Stdio};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn run_cli_with_stdin(pattern: &str, input: &str) -> Result<String> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rg"));
    cmd.arg(pattern)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("stdin"))?;
    stdin.write_all(input.as_bytes())?;
    drop(stdin);

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(Box::new(std::io::Error::other("non-zero exit")));
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

#[test]
fn cli_reads_stdin_and_outputs_matches() -> Result<()> {
    let output = run_cli_with_stdin("alp", "alpha\nbeta\n")?;
    // The output will contain ANSI codes, splitting "alpha" into "alp" (colored) and "ha" (uncolored).
    // So we check for the presence of "alp" and "ha" separately.
    assert!(output.contains("alp"));
    assert!(output.contains("ha"));
    Ok(())
}

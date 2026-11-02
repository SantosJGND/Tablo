mod common;

use common::{fixture, run, run_stdin, stdout, temp_file};
use std::io::Read;
use std::process::{Command, Stdio};

#[test]
fn top_level_style_grid_is_applied_to_default_head_mode() {
    let out = run(&[&fixture("semicolon.csv"), "--style", "grid"]);
    assert!(out.status.success());
    assert!(stdout(&out).starts_with("┌───────┬─────┬────────┐\n"));
}

#[test]
fn full_style_grid_is_applied() {
    let out = run(&["full", &fixture("semicolon.csv"), "--style", "grid"]);
    assert!(out.status.success());
    assert!(stdout(&out).starts_with("┌───────┬─────┬────────┐\n"));
}

#[test]
fn stdin_input_is_read_when_no_file_given() {
    let out = run_stdin(&["full"], b"a,b\n1,2\n");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "a  b\n1  2\n1 rows × 2 columns\n");
}

#[test]
fn missing_file_reports_error_and_exits_nonzero() {
    let out = run(&["full", "does_not_exist.csv"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("Failed to read file: does_not_exist.csv")
    );
}

#[test]
fn invalid_delimiter_reports_error_and_exits_nonzero() {
    let out = run(&[&fixture("semicolon.csv"), "-d", "xx"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Invalid delimiter: xx"));
}

#[test]
fn closed_stdout_pipe_exits_quietly() {
    let mut contents = String::from("id,value\n");
    for i in 0..300_000 {
        contents.push_str(&format!("{i},{}\n", i * 3));
    }
    let path = temp_file("broken_pipe.csv", contents.as_bytes());

    let mut child = Command::new(env!("CARGO_BIN_EXE_tablo"))
        .args(["full", path.to_str().unwrap()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn tablo");

    let mut stdout_pipe = child.stdout.take().expect("stdout");
    let mut first = [0u8; 1];
    stdout_pipe.read_exact(&mut first).expect("read first byte");
    drop(stdout_pipe);

    let output = child.wait_with_output().expect("wait tablo");
    let _ = std::fs::remove_file(&path);
    assert!(output.status.success(), "status: {:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

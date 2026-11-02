mod common;

use common::{fixture, run, stdout};
use std::process::Command;

fn has_escape(bytes: &[u8]) -> bool {
    bytes.contains(&0x1b)
}

#[test]
fn redirected_stdout_has_no_escape_sequences() {
    let out = run(&["full", &fixture("flat.jsonl")]);
    assert!(out.status.success());
    assert!(!has_escape(&out.stdout));
}

#[test]
fn style_flags_add_no_escape_sequences_when_redirected() {
    let out = run(&[
        "full",
        &fixture("flat.jsonl"),
        "--zebra",
        "--negative-highlight",
        "--style",
        "grid",
    ]);
    assert!(out.status.success());
    assert!(!has_escape(&out.stdout));
}

#[test]
fn style_flags_do_not_change_redirected_layout() {
    let plain = run(&["full", &fixture("flat.jsonl")]);
    let styled = run(&[
        "full",
        &fixture("flat.jsonl"),
        "--zebra",
        "--negative-highlight",
    ]);
    assert_eq!(stdout(&plain), stdout(&styled));
}

#[test]
fn no_color_flag_and_no_color_env_are_accepted() {
    let flag = run(&["full", &fixture("flat.jsonl"), "--no-color", "--zebra"]);
    assert!(flag.status.success());
    assert!(!has_escape(&flag.stdout));

    let env = Command::new(env!("CARGO_BIN_EXE_tablo"))
        .args(["full", &fixture("flat.jsonl"), "--zebra"])
        .env("NO_COLOR", "1")
        .output()
        .expect("run tablo");
    assert!(env.status.success());
    assert!(!has_escape(&env.stdout));
}

#[test]
fn head_mode_redirected_has_no_escape_sequences() {
    let out = run(&[
        &fixture("flat.jsonl"),
        "-n",
        "1",
        "--zebra",
        "--negative-highlight",
    ]);
    assert!(out.status.success());
    assert!(!has_escape(&out.stdout));
    assert_eq!(
        stdout(&out),
        "active  age  name \ntrue    31   Alice\n...\n"
    );
}

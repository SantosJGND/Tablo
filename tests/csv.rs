mod common;

use common::{fixture, run, run_stdin, stdout, temp_file};

#[test]
fn semicolon_delimiter_is_inferred() {
    let out = run(&["full", &fixture("semicolon.csv")]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "name   age  city  \nAlice  31   London\nBob    25   Paris \n2 rows × 3 columns\n"
    );
}

#[test]
fn explicit_delimiter_overrides_inference() {
    let out = run(&["full", &fixture("semicolon.csv"), "-d", ";"]);
    assert!(out.status.success());
    assert!(stdout(&out).ends_with("2 rows × 3 columns\n"));
}

#[test]
fn quoted_delimiters_escaped_quotes_and_embedded_newlines_are_parsed() {
    let out = run(&["full", &fixture("quoted.csv")]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("a, b"));
    assert!(text.contains("say \"hi\""));
    assert!(text.contains("line one\nline two"));
    assert!(text.ends_with("3 rows × 2 columns\n"));
}

#[test]
fn inconsistent_rows_are_padded_to_widest_row() {
    let out = run(&["full", &fixture("ragged.csv"), "--style", "grid"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "┌───┬───┬───┬───┐\n│ a │ b │ c │   │\n├───┼───┼───┼───┤\n│ 1 │ 2 │   │   │\n│ 3 │ 4 │ 5 │ 6 │\n└───┴───┴───┴───┘\n2 rows × 4 columns\n"
    );
}

#[test]
fn unicode_cells_are_aligned_by_display_width() {
    let out = run(&["full", &fixture("unicode.csv"), "--style", "grid"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "┌──────┬───────────┐\n│ name │ city      │\n├──────┼───────────┤\n│ José │ São Paulo │\n│ 李雷 │ 北京      │\n│ 😀   │ Zürich    │\n└──────┴───────────┘\n3 rows × 2 columns\n"
    );
}

#[test]
fn crlf_line_endings_are_handled() {
    let out = run_stdin(&["full"], b"a,b\r\n1,2\r\n");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "a  b\n1  2\n1 rows × 2 columns\n");
}

#[test]
fn empty_input_prints_nothing_in_head_and_zero_dimensions_in_full() {
    let head = run_stdin(&[], b"");
    assert!(head.status.success());
    assert_eq!(stdout(&head), "");

    let full = run_stdin(&["full"], b"");
    assert!(full.status.success());
    assert_eq!(stdout(&full), "0 rows × 0 columns\n");
}

#[test]
fn header_only_input_shows_header() {
    let out = run_stdin(&["full"], b"a,b\n");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "a  b\n0 rows × 2 columns\n");
}

#[test]
fn head_marks_truncation_when_more_rows_exist() {
    let out = run(&["head", &fixture("semicolon.csv"), "-n", "1"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "name   age  city  \nAlice  31   London\n...\n"
    );
}

#[test]
fn head_without_header_marks_truncation_for_exactly_one_extra_row() {
    let out = run_stdin(&["head", "-n", "2", "--no-header"], b"x\n1\n2\n");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "x\n1\n...\n");
}

#[test]
fn utf8_character_spanning_delimiter_sample_boundary_does_not_panic() {
    // Byte 8192 (end of delimiter sample) lands inside a 2-byte character.
    let contents = format!("a\nx{}\n", "é".repeat(5000));
    let path = temp_file("utf8_boundary.csv", contents.as_bytes());
    let out = run(&["full", path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(stdout(&out).ends_with("1 rows × 1 columns\n"));
}

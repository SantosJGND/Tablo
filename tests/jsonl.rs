mod common;

use common::{fixture, run, run_stdin, stdout, temp_file};

#[test]
fn flat_objects_render_with_keys_as_header() {
    let out = run(&["full", &fixture("flat.jsonl")]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "active  age  name   \ntrue    31   Alice  \nfalse   25   Bob    \ntrue    42   Charlie\n3 rows × 3 columns\n"
    );
}

#[test]
fn nested_objects_are_flattened_to_dotted_paths() {
    let out = run(&["full", &fixture("nested.jsonl")]);
    assert!(out.status.success());
    let text = stdout(&out);
    let header = text.lines().next().unwrap();
    assert!(header.contains("user.address.city"));
    assert!(header.contains("user.email"));
    assert!(header.contains("user.name"));
    assert!(text.contains("London"));
    assert!(text.ends_with("2 rows × 7 columns\n"));
}

#[test]
fn arrays_and_empty_objects_are_compact_json_text() {
    let out = run(&["full", &fixture("nested.jsonl")]);
    let text = stdout(&out);
    assert!(text.contains("[\"a\",\"b\"]"));
    assert!(text.contains("{}"));
}

#[test]
fn missing_keys_and_nulls_render_as_empty_cells() {
    let out = run(&["full", &fixture("sparse.jsonl")]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "code  id     name   score  extra\nx     1      Alice  1.5         \n      2      Bob                \ny     three                true \n3 rows × 5 columns\n"
    );
}

#[test]
fn mixed_types_in_one_column_widen_to_string_keeping_text() {
    // `id` is an integer in two records and "three" in one.
    let out = run(&["full", &fixture("sparse.jsonl")]);
    let text = stdout(&out);
    assert!(text.contains("three"));
    assert!(text.contains("1 "));
    assert!(text.contains("2 "));
}

#[test]
fn head_marks_truncation_for_jsonl() {
    let out = run(&[&fixture("flat.jsonl"), "-n", "1"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "active  age  name \ntrue    31   Alice\n...\n"
    );
}

#[test]
fn unicode_values_align_by_display_width() {
    let out = run(&["full", &fixture("unicode.jsonl"), "--style", "grid"]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        "┌───────────┬──────┐\n│ city      │ name │\n├───────────┼──────┤\n│ São Paulo │ José │\n│ 北京      │ 李雷 │\n│ Zürich    │ 😀   │\n└───────────┴──────┘\n3 rows × 2 columns\n"
    );
}

#[test]
fn malformed_line_reports_line_number_and_exits_nonzero() {
    let out = run(&["full", &fixture("malformed.jsonl")]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("line 2"), "stderr: {err}");
    assert!(err.contains("invalid JSON"), "stderr: {err}");
}

#[test]
fn non_object_line_is_rejected_with_line_number() {
    let path = temp_file("array_line.jsonl", b"[1,2]\n");
    let out = run(&["full", path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("line 1: expected a JSON object"));
}

#[test]
fn empty_file_has_zero_dimensions() {
    let path = temp_file("empty.jsonl", b"");
    let out = run(&["full", path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "0 rows × 0 columns\n");
}

#[test]
fn blank_lines_are_skipped() {
    let out = run_stdin(
        &["full", "--format", "jsonl"],
        b"{\"a\":1}\n\n   \n{\"a\":2}\n",
    );
    assert!(out.status.success());
    assert_eq!(stdout(&out), "a\n1\n2\n2 rows × 1 columns\n");
}

#[test]
fn stdin_requires_format_flag_to_be_read_as_jsonl() {
    let out = run_stdin(&["--format", "jsonl"], b"{\"name\":\"Alice\"}\n");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "name \nAlice\n");
}

#[test]
fn ndjson_extension_is_detected() {
    let path = temp_file("records.ndjson", b"{\"k\":\"v\"}\n");
    let out = run(&["full", path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "k\nv\n1 rows × 1 columns\n");
}

#[test]
fn explicit_csv_format_overrides_jsonl_extension() {
    let path = temp_file("as_csv.jsonl", b"a,b\n1,2\n");
    let out = run(&["full", path.to_str().unwrap(), "--format", "csv"]);
    let _ = std::fs::remove_file(&path);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "a  b\n1  2\n1 rows × 2 columns\n");
}

#[test]
fn delimiter_is_rejected_for_jsonl() {
    let out = run(&[&fixture("flat.jsonl"), "-d", ","]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("--delimiter does not apply to JSONL input")
    );
}

#[test]
fn keys_first_seen_after_sample_are_dropped() {
    // Sample is the first 1000 records; `late` first appears on record 1001.
    let mut contents = String::new();
    for i in 0..1000 {
        contents.push_str(&format!("{{\"a\":{i}}}\n"));
    }
    contents.push_str("{\"a\":1,\"late\":2}\n");
    let path = temp_file("late_keys.jsonl", contents.as_bytes());
    let out = run(&["full", path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(!text.contains("late"));
    assert!(text.ends_with("1001 rows × 1 columns\n"));
}

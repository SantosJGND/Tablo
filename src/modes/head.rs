use super::{collect_rows, header_names, make_table, open, render};
use crate::cli::Style;
use anyhow::Result;

pub fn run(
    file: Option<&str>,
    delimiter: Option<&str>,
    lines: usize,
    no_header: bool,
    style: &Style,
) -> Result<String> {
    let mut source = open(file, delimiter, !no_header)?;
    let headers = header_names(source.as_ref());

    // One record beyond the preview tells us whether more rows exist.
    let rows = collect_rows(source.as_mut(), lines.saturating_add(1))?;
    let has_more = rows.len() > lines;

    let mut table = make_table(headers, rows);
    table.rows.truncate(lines);

    let mut output = render(style, &table);
    if has_more {
        output.push_str("...\n");
    }
    Ok(output)
}

use super::{collect_rows, header_names, make_table, open, render};
use crate::cli::{Format, Style};
use crate::output::style::Palette;
use anyhow::Result;

pub fn run(
    file: Option<&str>,
    delimiter: Option<&str>,
    no_header: bool,
    style: &Style,
    format: Option<&Format>,
    palette: Palette,
) -> Result<String> {
    let mut source = open(file, delimiter, !no_header, format)?;
    let headers = header_names(source.as_ref());
    let rows = collect_rows(source.as_mut(), usize::MAX)?;
    let table = make_table(headers, rows);

    let mut output = render(style, &table, &palette);

    // Add dataset dimensions
    let num_rows = table.rows.len();
    let num_cols = if let Some(ref headers) = table.headers {
        headers.len()
    } else if !table.rows.is_empty() {
        table.rows[0].len()
    } else {
        0
    };
    output.push_str(&format!("{} rows × {} columns\n", num_rows, num_cols));

    Ok(output)
}

pub mod full;
pub mod head;

use crate::cli::{Format, Style};
use crate::output::{grid, plain};
use crate::source::{open_source, parse_delimiter, DataSource, Input, SourceOptions};
use crate::table::Table;
use anyhow::Result;
use std::path::PathBuf;

pub fn open(
    file: Option<&str>,
    delimiter: Option<&str>,
    has_header: bool,
    format: Option<&Format>,
) -> Result<Box<dyn DataSource>> {
    let input = match file {
        Some(path) => Input::File(PathBuf::from(path)),
        None => Input::Stdin,
    };
    let delimiter = delimiter.map(parse_delimiter).transpose()?;
    open_source(
        input,
        SourceOptions {
            format: format.map(crate::source::Format::from),
            delimiter,
            has_header,
        },
    )
}

pub fn header_names(source: &dyn DataSource) -> Option<Vec<String>> {
    let has_header = source.metadata().has_header;
    let schema = source.schema();
    if has_header && !schema.fields.is_empty() {
        Some(schema.fields.iter().map(|f| f.name.clone()).collect())
    } else {
        None
    }
}

/// Reads at most `limit` records as display strings.
pub fn collect_rows(source: &mut dyn DataSource, limit: usize) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    while rows.len() < limit {
        let Some(record) = source.next_record()? else {
            break;
        };
        rows.push(record.values.iter().map(ToString::to_string).collect());
    }
    Ok(rows)
}

/// Pads headers and all rows to a common width.
pub fn make_table(headers: Option<Vec<String>>, mut rows: Vec<Vec<String>>) -> Table {
    let width = headers
        .iter()
        .map(Vec::len)
        .chain(rows.iter().map(Vec::len))
        .max()
        .unwrap_or(0);

    let headers = headers.map(|mut h| {
        h.resize(width, String::new());
        h
    });
    for row in &mut rows {
        row.resize(width, String::new());
    }

    Table { headers, rows }
}

pub fn render(style: &Style, table: &Table) -> String {
    match style {
        Style::Plain => plain::render(table),
        Style::Grid => grid::render(table),
    }
}

use crate::detect::{detect_delimiter, parse_delimiter};
use crate::input::read_input;
use crate::output::{grid, plain};
use crate::parser::parse_table;
use crate::table::Table;
use anyhow::Result;

pub fn run(
    file: Option<&str>,
    delimiter: Option<&str>,
    no_header: bool,
    style: &crate::cli::Style,
) -> Result<String> {
    let input = read_input(file)?;
    
    let delim = if let Some(d) = delimiter {
        parse_delimiter(d)?
    } else {
        detect_delimiter(&input)?
    };
    
    let table = parse_table(&input, delim, !no_header)?;
    
    let mut output = match style {
        crate::cli::Style::Plain => plain::render(&table),
        crate::cli::Style::Grid => grid::render(&table),
    };
    
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

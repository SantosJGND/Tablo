use crate::detect::{detect_delimiter, parse_delimiter};
use crate::input::read_input;
use crate::output::plain;
use crate::table::Table;
use anyhow::Result;
use csv::ReaderBuilder;
use std::io::{self, Read};

pub fn run(
    file: Option<&str>,
    delimiter: Option<&str>,
    lines: usize,
    no_header: bool,
) -> Result<String> {
    // For head mode, we need to stream to detect "more rows exist"
    let input = read_input(file)?;
    
    let delim = if let Some(d) = delimiter {
        parse_delimiter(d)?
    } else {
        detect_delimiter(&input)?
    };
    
    let mut reader = ReaderBuilder::new()
        .delimiter(delim)
        .has_headers(false)
        .flexible(true)
        .from_reader(input.as_bytes());
    
    let mut all_records: Vec<Vec<String>> = Vec::new();
    let mut record_count = 0;
    let mut has_more = false;
    
    for record in reader.records() {
        record_count += 1;
        // We need one extra to know if there are more
        if record_count > lines + 1 {
            has_more = true;
            break;
        }
        let rec = record?;
        let fields: Vec<String> = rec.iter().map(|s| s.to_string()).collect();
        all_records.push(fields);
    }
    
    if all_records.is_empty() {
        return Ok(String::new());
    }
    
    // Normalize lengths
    let max_len = all_records.iter().map(|r| r.len()).max().unwrap_or(0);
    for rec in &mut all_records {
        while rec.len() < max_len {
            rec.push(String::new());
        }
    }
    
    let table = if !no_header && !all_records.is_empty() {
        let headers = all_records.remove(0);
        let rows: Vec<Vec<String>> = all_records.into_iter().take(lines).collect();
        Table {
            headers: Some(headers),
            rows,
        }
    } else {
        let rows: Vec<Vec<String>> = all_records.into_iter().take(lines).collect();
        Table {
            headers: None,
            rows,
        }
    };
    
    let mut output = plain::render(&table);
    if has_more {
        output.push_str("...\n");
    }
    Ok(output)
}

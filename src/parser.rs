use crate::table::Table;
use anyhow::Result;
use csv::ReaderBuilder;

pub fn parse_table(input: &str, delimiter: u8, has_header: bool) -> Result<Table> {
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_reader(input.as_bytes());
    
    let mut all_records: Vec<Vec<String>> = Vec::new();
    for record in reader.records() {
        let rec = record?;
        let fields: Vec<String> = rec.iter().map(|s| s.to_string()).collect();
        all_records.push(fields);
    }
    
    if all_records.is_empty() {
        return Ok(Table {
            headers: None,
            rows: vec![],
        });
    }
    
    // Normalize inconsistent record lengths - pad shorter with empty strings
    let max_len = all_records.iter().map(|r| r.len()).max().unwrap_or(0);
    for rec in &mut all_records {
        while rec.len() < max_len {
            rec.push(String::new());
        }
    }
    
    if has_header && !all_records.is_empty() {
        let headers = all_records.remove(0);
        Ok(Table {
            headers: Some(headers),
            rows: all_records,
        })
    } else {
        Ok(Table {
            headers: None,
            rows: all_records,
        })
    }
}

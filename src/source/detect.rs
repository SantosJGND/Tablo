use anyhow::{Context, Result};
use std::io::{Cursor, Read};

const SAMPLE_BYTES: u64 = 8192;

/// Reads a bounded sample, detects the delimiter from it, and returns a reader
/// that replays the sample followed by the rest of the input (no buffering of
/// the whole input).
pub fn sniff(mut input: Box<dyn Read>) -> Result<(u8, Box<dyn Read>)> {
    let mut sample = Vec::new();
    input
        .by_ref()
        .take(SAMPLE_BYTES)
        .read_to_end(&mut sample)
        .context("Failed to read input")?;
    let delimiter = detect_delimiter(&sample);
    Ok((delimiter, Box::new(Cursor::new(sample).chain(input))))
}

pub fn detect_delimiter(sample: &[u8]) -> u8 {
    let candidates = [b',', b';', b'\t', b'|'];
    let mut best_delim = b',';
    let mut best_score = 0i32;

    for &delim in &candidates {
        if let Some(score) = score_delimiter(sample, delim) {
            if score > best_score {
                best_score = score;
                best_delim = delim;
            }
        }
    }

    // best_delim already falls back to ',' when no candidate scored.
    best_delim
}

fn score_delimiter(sample: &[u8], delim: u8) -> Option<i32> {
    // Use CSV parser rules to avoid splitting quoted fields
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delim)
        .has_headers(false)
        .flexible(true)
        .from_reader(sample);

    let mut records: Vec<Vec<String>> = Vec::new();
    for rec in reader.records().take(10).flatten() {
        let fields: Vec<String> = rec.iter().map(|s| s.to_string()).collect();
        if !fields.is_empty() {
            records.push(fields);
        }
    }

    if records.is_empty() {
        return None;
    }

    // Reject single-column output as not convincing
    let avg_cols = records.iter().map(|r| r.len()).sum::<usize>() as f32 / records.len() as f32;
    if avg_cols < 1.5 {
        return None;
    }

    // Calculate consistency
    let first_len = records[0].len();
    let mut consistent = 0;
    let mut total_variance = 0i32;
    for rec in &records {
        if rec.len() == first_len {
            consistent += 1;
        }
        total_variance += ((rec.len() as i32) - (first_len as i32)).abs();
    }

    let consistency_ratio = consistent as f32 / records.len() as f32;
    let score = ((avg_cols * 10.0) as i32) + ((consistency_ratio * 100.0) as i32) - total_variance;

    Some(score)
}

pub fn parse_delimiter(delim_str: &str) -> Result<u8> {
    match delim_str {
        "tab" | "\\t" => Ok(b'\t'),
        s if s.len() == 1 => Ok(s.as_bytes()[0]),
        _ => anyhow::bail!("Invalid delimiter: {}", delim_str),
    }
}

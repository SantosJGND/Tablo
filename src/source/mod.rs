// The source contract includes items that later phases consume (capabilities
// for projection and row counts, format name for schema display).
#![allow(dead_code)]

mod csv;
mod detect;

use crate::data::{Record, Schema};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub use detect::parse_delimiter;

pub trait DataSource {
    fn schema(&self) -> &Schema;
    fn next_record(&mut self) -> Result<Option<Record>>;
    fn metadata(&self) -> SourceMetadata;
    fn capabilities(&self) -> SourceCapabilities;
}

#[derive(Debug, Clone)]
pub struct SourceMetadata {
    pub format: &'static str,
    pub has_header: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SourceCapabilities {
    pub streaming: bool,
    pub projection: bool,
    pub known_schema: bool,
    pub known_row_count: bool,
}

pub enum Input {
    File(PathBuf),
    Stdin,
}

#[derive(Debug, Default)]
pub struct SourceOptions {
    pub delimiter: Option<u8>,
    pub has_header: bool,
}

pub fn open_source(input: Input, options: SourceOptions) -> Result<Box<dyn DataSource>> {
    let reader: Box<dyn Read> = match input {
        Input::File(path) => Box::new(
            File::open(&path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?,
        ),
        Input::Stdin => Box::new(io::stdin()),
    };

    let (delimiter, reader) = match options.delimiter {
        Some(delimiter) => (delimiter, reader),
        None => detect::sniff(reader)?,
    };

    Ok(Box::new(csv::CsvSource::new(
        reader,
        delimiter,
        options.has_header,
    )?))
}

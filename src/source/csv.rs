use super::{DataSource, SourceCapabilities, SourceMetadata};
use crate::data::{DataType, Field, Record, Schema, Value};
use anyhow::Result;
use csv::{Reader, ReaderBuilder};
use std::io::Read;

pub struct CsvSource {
    reader: Reader<Box<dyn Read>>,
    schema: Schema,
    has_header: bool,
    /// First record when there is no header row; it is yielded as data.
    pending: Option<Vec<String>>,
}

impl CsvSource {
    pub fn new(input: Box<dyn Read>, delimiter: u8, has_header: bool) -> Result<Self> {
        let mut reader = ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(false)
            .flexible(true)
            .from_reader(input);

        let first = read_fields(&mut reader)?;
        let (names, pending) = match first {
            None => (Vec::new(), None),
            Some(fields) if has_header => (fields, None),
            Some(fields) => (
                (1..=fields.len()).map(|i| format!("column_{i}")).collect(),
                Some(fields),
            ),
        };

        let schema = Schema {
            fields: names
                .into_iter()
                .map(|name| Field {
                    name,
                    data_type: DataType::String,
                    nullable: true,
                })
                .collect(),
        };

        Ok(Self {
            reader,
            schema,
            has_header,
            pending,
        })
    }
}

fn read_fields(reader: &mut Reader<Box<dyn Read>>) -> Result<Option<Vec<String>>> {
    match reader.records().next() {
        None => Ok(None),
        Some(record) => Ok(Some(record?.iter().map(str::to_owned).collect())),
    }
}

impl DataSource for CsvSource {
    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn next_record(&mut self) -> Result<Option<Record>> {
        let fields = match self.pending.take() {
            Some(fields) => Some(fields),
            None => read_fields(&mut self.reader)?,
        };
        Ok(fields.map(|fields| Record {
            values: fields.into_iter().map(Value::String).collect(),
        }))
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            format: "csv",
            has_header: self.has_header,
        }
    }

    fn capabilities(&self) -> SourceCapabilities {
        SourceCapabilities {
            streaming: true,
            projection: false,
            known_schema: false,
            known_row_count: false,
        }
    }
}

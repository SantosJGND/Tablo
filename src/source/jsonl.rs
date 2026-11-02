use super::{DataSource, SourceCapabilities, SourceMetadata};
use crate::data::{DataType, Field, Record, Schema, Value};
use anyhow::{bail, Context, Result};
use serde_json::{Map, Value as Json};
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};

/// Number of non-blank records read up front to infer the schema.
const SAMPLE_RECORDS: usize = 1000;

/// Flattened (dotted path, value) pairs for one record.
type Fields = Vec<(String, Value)>;

pub struct JsonlSource {
    reader: BufReader<Box<dyn Read>>,
    line_no: usize,
    schema: Schema,
    index: HashMap<String, usize>,
    /// Sampled records not yet yielded.
    pending: VecDeque<Fields>,
}

impl JsonlSource {
    pub fn new(input: Box<dyn Read>) -> Result<Self> {
        let mut source = Self {
            reader: BufReader::new(input),
            line_no: 0,
            schema: Schema::default(),
            index: HashMap::new(),
            pending: VecDeque::new(),
        };

        let mut sample = Vec::new();
        while sample.len() < SAMPLE_RECORDS {
            match source.read_record()? {
                Some(fields) => sample.push(fields),
                None => break,
            }
        }

        let (schema, index) = infer_schema(&sample);
        source.schema = schema;
        source.index = index;
        source.pending = VecDeque::from(sample);
        Ok(source)
    }

    /// Reads the next non-blank line as a flattened object.
    fn read_record(&mut self) -> Result<Option<Fields>> {
        loop {
            let mut line = String::new();
            let read = self
                .reader
                .read_line(&mut line)
                .with_context(|| format!("line {}: failed to read input", self.line_no + 1))?;
            if read == 0 {
                return Ok(None);
            }
            self.line_no += 1;

            let text = line.trim();
            if text.is_empty() {
                continue;
            }

            let value: Json = serde_json::from_str(text).map_err(|err| {
                anyhow::anyhow!(
                    "line {}, column {}: invalid JSON ({})",
                    self.line_no,
                    err.column(),
                    describe(err.classify())
                )
            })?;

            let Json::Object(map) = value else {
                bail!("line {}: expected a JSON object", self.line_no);
            };
            let mut fields = Vec::new();
            for (key, value) in map {
                flatten_into(key, value, &mut fields);
            }
            return Ok(Some(fields));
        }
    }

    /// Places values in schema order. Missing keys become `Null`; keys not
    /// seen during inference are dropped.
    fn project(&self, fields: Fields) -> Record {
        let mut values = vec![Value::Null; self.schema.fields.len()];
        for (name, value) in fields {
            if let Some(&i) = self.index.get(&name) {
                values[i] = coerce(value, self.schema.fields[i].data_type);
            }
        }
        Record { values }
    }
}

impl DataSource for JsonlSource {
    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn next_record(&mut self) -> Result<Option<Record>> {
        let fields = match self.pending.pop_front() {
            Some(fields) => Some(fields),
            None => self.read_record()?,
        };
        Ok(fields.map(|fields| self.project(fields)))
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            format: "jsonl",
            has_header: true,
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

fn describe(category: serde_json::error::Category) -> &'static str {
    use serde_json::error::Category;
    match category {
        Category::Eof => "unexpected end of line",
        Category::Syntax => "syntax error",
        Category::Data => "invalid data",
        Category::Io => "read error",
    }
}

fn flatten_into(path: String, value: Json, out: &mut Fields) {
    match value {
        Json::Object(map) if !map.is_empty() => flatten_map(&path, map, out),
        Json::Object(_) => out.push((path, Value::Json("{}".to_string()))),
        other => out.push((path, to_value(other))),
    }
}

fn flatten_map(prefix: &str, map: Map<String, Json>, out: &mut Fields) {
    for (key, value) in map {
        flatten_into(format!("{prefix}.{key}"), value, out);
    }
}

fn to_value(value: Json) -> Value {
    match value {
        Json::Null => Value::Null,
        Json::Bool(b) => Value::Boolean(b),
        Json::Number(n) => match n.as_i64() {
            Some(i) => Value::Integer(i),
            // Beyond i64 but a float literal: keep as Float.
            None if n.is_f64() => Value::Float(n.as_f64().unwrap_or(f64::NAN)),
            // Integer beyond i64: keep the exact text.
            None => Value::String(n.to_string()),
        },
        Json::String(s) => Value::String(s),
        // Arrays and any other nested shape: compact JSON text.
        other => Value::Json(other.to_string()),
    }
}

fn value_type(value: &Value) -> Option<DataType> {
    match value {
        Value::Null => None,
        Value::Boolean(_) => Some(DataType::Boolean),
        Value::Integer(_) => Some(DataType::Integer),
        Value::Float(_) => Some(DataType::Float),
        Value::String(_) => Some(DataType::String),
        Value::Json(_) => Some(DataType::Json),
    }
}

/// Integer and Float widen to Float; any other mix widens to String.
fn merge(a: DataType, b: DataType) -> DataType {
    if a == b {
        return a;
    }
    match (a, b) {
        (DataType::Integer, DataType::Float) | (DataType::Float, DataType::Integer) => {
            DataType::Float
        }
        _ => DataType::String,
    }
}

/// Converts a value to the column's type where the conversion is lossless
/// enough for display; other values keep their own type.
fn coerce(value: Value, column: DataType) -> Value {
    match (column, value) {
        (DataType::String, Value::Integer(i)) => Value::String(i.to_string()),
        (DataType::String, Value::Float(x)) => Value::String(x.to_string()),
        (DataType::String, Value::Boolean(b)) => Value::String(b.to_string()),
        (DataType::Float, Value::Integer(i)) => Value::Float(i as f64),
        (_, value) => value,
    }
}

struct Column {
    name: String,
    data_type: Option<DataType>,
    /// Number of sampled records that contain this key.
    present: usize,
    /// Index of the last sampled record counted in `present`.
    last_record: usize,
    has_null: bool,
}

fn infer_schema(sample: &[Fields]) -> (Schema, HashMap<String, usize>) {
    let mut columns: Vec<Column> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();

    for (record_no, fields) in sample.iter().enumerate() {
        for (name, value) in fields {
            let i = *index.entry(name.clone()).or_insert_with(|| {
                columns.push(Column {
                    name: name.clone(),
                    data_type: None,
                    present: 0,
                    last_record: usize::MAX,
                    has_null: false,
                });
                columns.len() - 1
            });
            let column = &mut columns[i];
            if column.last_record != record_no {
                column.last_record = record_no;
                column.present += 1;
            }
            match value_type(value) {
                Some(t) => {
                    column.data_type = Some(match column.data_type {
                        None => t,
                        Some(prev) => merge(prev, t),
                    })
                }
                None => column.has_null = true,
            }
        }
    }

    let total = sample.len();
    let fields = columns
        .into_iter()
        .map(|column| Field {
            name: column.name,
            data_type: column.data_type.unwrap_or(DataType::Null),
            nullable: column.has_null || column.present < total,
        })
        .collect();
    (Schema { fields }, index)
}

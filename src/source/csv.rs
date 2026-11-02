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
            values: fields.into_iter().map(infer_value).collect(),
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

/// Conservative inference for a CSV cell. A value is typed only when its
/// canonical text equals the original, so display text never changes:
/// `00123`, `1.50`, `1e5`, `inf` and `NaN` stay strings.
fn infer_value(text: String) -> Value {
    if text.is_empty() {
        return Value::Null;
    }
    match text.as_str() {
        "true" => return Value::Boolean(true),
        "false" => return Value::Boolean(false),
        _ => {}
    }
    if let Ok(i) = text.parse::<i64>() {
        if i.to_string() == text {
            return Value::Integer(i);
        }
    }
    if let Ok(x) = text.parse::<f64>() {
        if x.is_finite() && x.to_string() == text {
            return Value::Float(x);
        }
    }
    Value::String(text)
}

#[cfg(test)]
mod tests {
    use super::infer_value;
    use crate::data::Value;

    #[test]
    fn classifies_conservatively() {
        assert_eq!(infer_value("true".into()), Value::Boolean(true));
        assert_eq!(infer_value("false".into()), Value::Boolean(false));
        assert_eq!(infer_value("42".into()), Value::Integer(42));
        assert_eq!(infer_value("-3".into()), Value::Integer(-3));
        assert_eq!(infer_value("1.5".into()), Value::Float(1.5));
        assert_eq!(infer_value("".into()), Value::Null);
    }

    #[test]
    fn non_canonical_text_stays_string() {
        for text in ["00123", "1.50", "1e5", "inf", "NaN", "+1", "True", "-0.0"] {
            assert_eq!(
                infer_value(text.into()),
                Value::String(text.into()),
                "{text}"
            );
        }
    }

    #[test]
    fn display_text_is_always_preserved() {
        for text in ["00123", "1.50", "42", "-3", "1.5", "true", "", "x y", "1e5"] {
            assert_eq!(infer_value(text.into()).to_string(), text);
        }
    }
}

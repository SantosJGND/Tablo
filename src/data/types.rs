use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    /// Arrays and empty objects, kept as compact JSON text.
    Json,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    /// Compact JSON text.
    Json(String),
}

/// Display text used by renderers. `Null` renders as an empty cell.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => Ok(()),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(x) => write!(f, "{x}"),
            Value::String(s) | Value::Json(s) => f.write_str(s),
        }
    }
}

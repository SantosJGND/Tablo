#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
}

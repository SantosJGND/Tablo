use super::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub values: Vec<Value>,
}

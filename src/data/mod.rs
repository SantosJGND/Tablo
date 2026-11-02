// The data model is ahead of its consumers (inference, schema display and
// Parquet types arrive in later phases), so unread items are allowed here.
#![allow(dead_code)]

pub mod record;
pub mod schema;
pub mod types;

pub use record::Record;
pub use schema::{Field, Schema};
pub use types::{DataType, Value};

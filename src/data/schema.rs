use super::types::DataType;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Schema {
    pub fields: Vec<Field>,
}

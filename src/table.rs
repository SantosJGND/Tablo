#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Option<Vec<String>>,
    pub rows: Vec<Vec<String>>,
}

/// Semantic category of a cell, used only for styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Text,
    Number { negative: bool },
    Boolean(bool),
    Null,
}

/// Display text plus the category used to style it. Widths are always
/// computed from `text`, which never contains ANSI sequences.
#[derive(Debug, Clone)]
pub struct Cell {
    pub text: String,
    pub kind: CellKind,
}

impl Cell {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: CellKind::Text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Option<Vec<String>>,
    pub rows: Vec<Vec<Cell>>,
}

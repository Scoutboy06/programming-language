#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: LineColumn::new(start_line, start_col),
            end: LineColumn::new(end_line, end_col),
        }
    }

    pub fn between(start: &LineColumn, end: &LineColumn) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub col: usize,
}

impl LineColumn {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

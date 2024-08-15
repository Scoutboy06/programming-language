use crate::parser::{Expression, Statement};
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Node {
    pub start: usize,
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

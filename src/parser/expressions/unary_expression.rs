use serde::Serialize;

use super::Expression;
use crate::{lexer::Span, parser::Node};

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub node: Node,
    pub kind: UnaryKind,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum UnaryKind {
    Not,
    Negative,
}

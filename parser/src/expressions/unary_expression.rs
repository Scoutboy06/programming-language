use super::Expression;
use crate::nodes::Node;

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub node: Node,
    pub kind: UnaryKind,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryKind {
    Not,
    Negative,
}

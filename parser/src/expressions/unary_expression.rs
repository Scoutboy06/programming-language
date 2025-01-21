use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
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

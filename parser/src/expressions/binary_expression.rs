use super::{BinaryOperation, Literal};
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryExpression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryOperation(BinaryOperation),
}

impl BinaryExpression {
    pub fn node(&self) -> &Node {
        match self {
            BinaryExpression::Literal(l) => l.node(),
            BinaryExpression::Identifier(i) => &i.node,
            BinaryExpression::BinaryOperation(b) => &b.node,
        }
    }
}

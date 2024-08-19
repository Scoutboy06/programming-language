use super::{Expression, Literal, NumberLiteral};
use crate::parser::{ArithmeticOperator, Identifier, Node};

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub node: Node,
    pub left: Box<BinaryExpression>,
    pub right: Box<BinaryExpression>,
    pub operator: ArithmeticOperator,
}

#[derive(Debug, PartialEq)]
pub enum BinaryExpression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryOperation(BinaryOperation),
}

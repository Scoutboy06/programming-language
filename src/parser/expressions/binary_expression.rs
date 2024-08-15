use super::{Expression, Literal, NumberLiteral};
use crate::{
    lexer::Span,
    parser::{ArithmeticOperator, Node},
};

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
    BinaryOperation(BinaryOperation),
}

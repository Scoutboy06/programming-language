use super::Literal;
use crate::nodes::Node;
use crate::statements::Identifier;
use lexer::ArithmeticOperator;

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

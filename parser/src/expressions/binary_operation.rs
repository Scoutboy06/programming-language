use super::BinaryExpression;
use crate::nodes::Node;
use lexer::ArithmeticOperator;

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub node: Node,
    pub left: Box<BinaryExpression>,
    pub right: Box<BinaryExpression>,
    pub operator: ArithmeticOperator,
}

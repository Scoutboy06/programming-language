use super::Expression;
use crate::nodes::Node;
use lexer::ArithmeticOperator;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub operator: ArithmeticOperator,
}

impl From<BinaryExpression> for Expression {
    fn from(value: BinaryExpression) -> Self {
        Expression::BinaryExpression(Box::new(value))
    }
}

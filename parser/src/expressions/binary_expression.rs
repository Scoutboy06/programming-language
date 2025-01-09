use super::Expression;
use crate::nodes::Node;
use lexer::Operator;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub operator: Operator,
}

impl From<BinaryExpression> for Expression {
    fn from(value: BinaryExpression) -> Self {
        Expression::BinaryExpression(Box::new(value))
    }
}

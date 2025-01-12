use super::Expression;
use crate::nodes::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesisExpression {
    pub node: Node,
    pub expression: Expression,
}

impl From<ParenthesisExpression> for Expression {
    fn from(value: ParenthesisExpression) -> Self {
        Expression::ParenthesisExpression(Box::new(value))
    }
}

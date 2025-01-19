use crate::nodes::Node;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpression {
    pub node: Node,
    pub items: Vec<Expression>,
}

impl From<ArrayExpression> for Expression {
    fn from(value: ArrayExpression) -> Self {
        Expression::ArrayExpression(Box::new(value))
    }
}

use super::Expression;
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

impl From<CallExpression> for Expression {
    fn from(value: CallExpression) -> Self {
        Expression::CallExpression(Box::new(value))
    }
}

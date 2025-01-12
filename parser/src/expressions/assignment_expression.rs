use super::Expression;
use crate::nodes::Node;
use lexer::Operator;

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: Operator,
    pub left: Expression,
    pub right: Expression,
}

impl From<AssignmentExpression> for Expression {
    fn from(value: AssignmentExpression) -> Self {
        Expression::AssignmentExpression(Box::new(value))
    }
}

use super::Expression;
use crate::nodes::Node;
use lexer::AssignmentOperator;

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Expression,
    pub right: Expression,
}

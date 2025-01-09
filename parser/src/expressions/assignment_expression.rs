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

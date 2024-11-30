use super::Expression;
use crate::{nodes::Node, statements::Identifier};
use lexer::AssignmentOperator;

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Identifier,
    pub right: Expression,
}

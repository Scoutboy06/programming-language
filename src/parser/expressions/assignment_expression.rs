use crate::parser::{statements::Identifier, AssignmentOperator, Expression, Node};

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Identifier,
    pub right: Expression,
}

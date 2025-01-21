use super::Expression;
use crate::nodes::Node;
use lexer::Operator;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: Operator,
    pub left: Expression,
    pub right: Expression,
}

use super::Expression;
use crate::nodes::Node;
use lexer::Operator;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub operator: Operator,
}

use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct SequenceExpression {
    pub node: Node,
    pub expressions: Vec<Expression>,
}

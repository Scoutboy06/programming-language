use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct ThisExpression {
    pub node: Node,
}

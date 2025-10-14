use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct SuperExpression {
    pub node: Node,
}

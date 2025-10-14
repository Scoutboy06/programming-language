use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct YieldExpression {
    pub node: Node,
    pub argument: Option<Expression>,
    pub delegate: bool,
}

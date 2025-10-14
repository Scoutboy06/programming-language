use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct TernaryExpression {
    pub node: Node,
    pub truthy_expr: Box<Expression>,
    pub falsy_expr: Box<Expression>,
}

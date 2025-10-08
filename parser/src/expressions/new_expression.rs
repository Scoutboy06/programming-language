use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct NewExpression {
    pub node: Node,
    pub expr: Box<Expression>,
}

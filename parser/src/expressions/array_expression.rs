use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ArrayExpression {
    pub node: Node,
    pub items: Vec<Expression>,
}

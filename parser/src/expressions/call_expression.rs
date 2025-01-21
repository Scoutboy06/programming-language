use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

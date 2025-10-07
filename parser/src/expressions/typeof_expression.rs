use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct TypeofExpression {
    pub node: Node,
    pub expression: Expression,
}

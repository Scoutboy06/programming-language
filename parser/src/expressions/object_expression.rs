use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ObjectExpression {
    pub node: Node,
    pub items: Vec<KV>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    pub key: Expression,
    pub value: Expression,
}

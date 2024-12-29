use crate::nodes::Node;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpression {
    pub node: Node,
    pub items: Vec<KV>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    pub key: Expression,
    pub value: Expression,
}

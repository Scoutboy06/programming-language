use super::{ComputedProperty, Expression, Identifier};
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ObjectExpression {
    pub node: Node,
    pub items: Vec<ObjectItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectItem {
    KV(KV),
    Identifier(Identifier),
    Computed(ComputedProperty),
}

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    pub key: Expression,
    pub value: Expression,
}

impl From<KV> for ObjectItem {
    fn from(value: KV) -> Self {
        ObjectItem::KV(value)
    }
}

impl From<Identifier> for ObjectItem {
    fn from(value: Identifier) -> Self {
        ObjectItem::Identifier(value)
    }
}

impl From<ComputedProperty> for ObjectItem {
    fn from(value: ComputedProperty) -> Self {
        ObjectItem::Computed(value)
    }
}

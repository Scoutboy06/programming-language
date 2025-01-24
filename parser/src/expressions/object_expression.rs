use super::{ComputedProperty, Expression, Identifier, StringLiteral};
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    pub key: Key,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    Computed(ComputedProperty),
}

impl From<Identifier> for Key {
    fn from(value: Identifier) -> Self {
        Key::Identifier(value)
    }
}

impl From<StringLiteral> for Key {
    fn from(value: StringLiteral) -> Self {
        Key::StringLiteral(value)
    }
}

impl From<ComputedProperty> for Key {
    fn from(value: ComputedProperty) -> Self {
        Key::Computed(value)
    }
}

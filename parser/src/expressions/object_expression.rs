use super::{Expression, Identifier};
use crate::{expressions::Literal, nodes::Node};
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ObjectExpression {
    pub node: Node,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub node: Node,
    pub key: PropertyKey,
    pub value: Expression,
    pub kind: PropertyKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

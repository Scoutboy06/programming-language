use super::{
    types::{TypeAnnotation, TypeParameterDeclaration},
    ComputedProperty, Expression, Identifier, StringLiteral,
};
use crate::{
    impl_from,
    nodes::Node,
    statements::{BlockStatement, Parameter},
};
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
    Method(Method),
}
impl_from!(ObjectItem, KV);
impl_from!(ObjectItem, Identifier);
impl_from!(ObjectItem, Method);

#[derive(Debug, Clone, PartialEq)]
pub struct KV {
    pub key: Key,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub node: Node,
    pub is_async: bool,
    pub is_generator: bool,
    pub id: Identifier,
    pub type_parameters: Option<TypeParameterDeclaration>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    ComputedProperty(ComputedProperty),
}
impl_from!(Key, Identifier);
impl_from!(Key, StringLiteral);
impl_from!(Key, ComputedProperty);

impl Key {
    pub fn node(&self) -> &Node {
        match self {
            Self::Identifier(id) => &id.node,
            Self::ComputedProperty(pr) => &pr.node,
            Self::StringLiteral(st) => &st.node,
        }
    }
}

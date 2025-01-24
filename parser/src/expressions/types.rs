use super::Identifier;
use crate::{impl_from, nodes::Node};
use lexer::TypeKeyword;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameterDeclaration {
    pub node: Node,
    pub parameters: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub node: Node,
    pub id: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub node: Node,
    pub type_value: TypeValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeValue {
    KeywordType(Box<KeywordType>),
    TypeReference(Box<TypeReference>),
    ArrayType(Box<ArrayType>),
    TypeLiteral(Box<TypeLiteral>),
}

impl TypeValue {
    pub fn node(&self) -> &Node {
        match self {
            Self::KeywordType(v) => &v.node,
            Self::TypeReference(v) => &v.node,
            Self::ArrayType(v) => &v.node,
            Self::TypeLiteral(v) => &v.node,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeywordType {
    pub node: Node,
    pub kind: TypeKeyword,
}
impl_from!(TypeValue, KeywordType);

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub node: Node,
    pub type_name: Identifier,
    pub type_params: Option<Vec<TypeValue>>,
}
impl_from!(TypeValue, TypeReference);

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub node: Node,
    pub type_value: TypeValue,
}
impl_from!(TypeValue, ArrayType);

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteral {
    pub node: Node,
}
impl_from!(TypeValue, TypeLiteral);

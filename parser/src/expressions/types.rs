use lexer::TypeKeyword;

use crate::{nodes::Node, statements::Identifier};

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
    KeywordType(KeywordType),
    TypeReference(TypeReference),
    ArrayType(ArrayType),
    TypeLiteral(TypeLiteral),
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

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub node: Node,
    pub type_name: Identifier,
    pub type_params: Option<Vec<TypeValue>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    node: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteral {
    node: Node,
}

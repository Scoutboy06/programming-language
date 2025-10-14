use super::{Identifier, Literal};
use crate::{impl_from, nodes::Node, statements::Parameter};
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
    pub type_value: AstType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    KeywordType(Box<KeywordType>),
    TypeReference(Box<TypeReference>),
    ArrayType(Box<ArrayType>),
    FnType(Box<FnType>),
    TypeLiteral(Box<TypeLiteral>),
}

impl AstType {
    pub fn node(&self) -> &Node {
        match self {
            Self::KeywordType(v) => &v.node,
            Self::TypeReference(v) => &v.node,
            Self::ArrayType(v) => &v.node,
            Self::FnType(v) => &v.node,
            Self::TypeLiteral(v) => &v.node,
        }
    }

    /// Checks if two `AstType`s structurally match, allowing for some leniency in optional type parameters
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KeywordType(s), Self::KeywordType(o)) => s.kind == o.kind,
            (Self::TypeReference(s), Self::TypeReference(o)) => {
                if s.type_name != o.type_name {
                    return false;
                }
                Self::match_type_params(s.type_params.as_deref(), o.type_params.as_deref())
            }
            (Self::ArrayType(s), Self::ArrayType(o)) => s.type_value == o.type_value,
            (Self::TypeLiteral(s), Self::TypeLiteral(o)) => s == o,
            _ => false,
        }
    }

    fn match_type_params(a: Option<&[Self]>, b: Option<&[Self]>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a.len() == b.len() && a.iter().zip(b).all(|(l, r)| l.matches(r)),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeywordType {
    pub node: Node,
    pub kind: TypeKeyword,
}
impl_from!(AstType, KeywordType);

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub node: Node,
    pub type_name: Identifier,
    pub type_params: Option<Vec<AstType>>,
}
impl_from!(AstType, TypeReference);

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub node: Node,
    pub type_value: AstType,
}
impl_from!(AstType, ArrayType);

#[derive(Debug, Clone, PartialEq)]
pub struct FnType {
    pub node: Node,
    pub params: Vec<Parameter>,
    pub return_type: TypeAnnotation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteral {
    pub node: Node,
    pub literal: Literal,
}
impl_from!(AstType, TypeLiteral);

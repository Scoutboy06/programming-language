use std::collections::HashMap;

use lexer::TypeKeyword;
use parser::{
    expressions::{types::TypeValue, Expression, Literal},
    nodes::Node,
};
use string_cache::DefaultAtom as Atom;

pub struct Symbol {
    pub id: Atom,
    pub unfolded_type: Option<SymbolKind>,
    pub display_type: Option<TypeValue>,
    pub declared_at: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Unknown,
    Number,
    String,
    Boolean,
    Null,
    Array(Box<Self>),
    Object(Box<ObjectType>),
    Union(Vec<Self>),
    Function(()),
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "{{unknown}}"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Null => write!(f, "null"),
            Self::Array(inner) => match **inner {
                Self::Union(_) | Self::Function(_) => write!(f, "({})[]", inner),
                _ => write!(f, "{}[]", inner),
            },
            Self::Object(object_type) => write!(
                f,
                "Record<{}, {}>",
                object_type.key_type, object_type.value_type
            ),
            Self::Union(symbol_kinds) => {
                let out = symbol_kinds
                    .iter()
                    .map(|kind| kind.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ");
                write!(f, "{}", out)
            }
            Self::Function(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub key_type: SymbolKind,
    pub value_type: SymbolKind,
}

impl SymbolKind {
    pub fn matches(&self, ann_type: &TypeValue) -> bool {
        match ann_type {
            TypeValue::TypeLiteral(type_literal) => match type_literal.literal {
                Literal::BooleanLiteral(_) => *self == Self::Boolean,
                Literal::NumberLiteral(_) => *self == Self::Number,
                Literal::NullLiteral(_) => *self == Self::Null,
                Literal::StringLiteral(_) => *self == Self::String,
            },
            TypeValue::KeywordType(keyword_type) => match keyword_type.kind {
                TypeKeyword::String => *self == Self::String,
                TypeKeyword::Number => *self == Self::Number,
                TypeKeyword::Boolean => *self == Self::Boolean,
            },
            TypeValue::TypeReference(_type_reference) => todo!(),
            TypeValue::ArrayType(_array_type) => todo!(),
        }
    }
}

pub struct SymbolTable {
    pub scopes: Vec<HashMap<Atom, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add(
        &mut self,
        id: Atom,
        unfolded_type: Option<SymbolKind>,
        display_type: Option<TypeValue>,
        declared_at: Node,
    ) {
        debug_assert!(self.scopes.len() > 0);
        let a = self.scopes.last_mut().unwrap();
        a.insert(
            id.clone(),
            Symbol {
                id,
                unfolded_type,
                display_type,
                declared_at,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ObjectType, SymbolKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn format_union() {
        use SymbolKind as T;
        let t = T::Union([T::String, T::Number, T::Null].to_vec());
        assert_eq!(t.to_string(), "string | number | null".to_string());
    }

    #[test]
    fn format_object() {
        use SymbolKind as T;
        let t = T::Object(Box::new(ObjectType {
            key_type: T::String,
            value_type: T::Union([T::Number, T::Null].to_vec()),
        }));
        assert_eq!(t.to_string(), "Record<string, number | null>".to_string());
    }
}

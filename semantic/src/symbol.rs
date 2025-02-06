use std::collections::HashMap;

use lexer::TypeKeyword;
use parser::{
    expressions::{types::TypeValue, Literal},
    nodes::Node,
};
use string_cache::DefaultAtom as Atom;

use crate::{CheckerContext, ErrorSeverity};

pub struct Symbol {
    pub id: Atom,
    pub unfolded_type: Option<ExprType>,
    pub display_type: Option<TypeValue>,
    pub declared_at: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
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

pub trait ExprTypeIncludes {
    fn includes(&self, t: &ExprType) -> bool;
}

impl ExprType {
    pub fn extend(&mut self, other: &Self) {
        match self {
            Self::Unknown => *self = other.clone(),
            Self::Union(types) => {
                if !types.contains(other) {
                    types.push(other.clone());
                }
            }
            _ if self != other => *self = Self::Union([self.clone(), other.clone()].to_vec()),
            _ => {}
        }
    }
}
impl ExprTypeIncludes for ExprType {
    fn includes(&self, t: &ExprType) -> bool {
        match self {
            Self::Union(u) => u.contains(t),
            _ => *self == *t,
        }
    }
}

impl std::fmt::Display for ExprType {
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
    pub key_type: ExprType,
    pub value_type: ExprType,
}

impl ExprType {
    pub fn matches(&self, ann_type: &TypeValue, ctx: &mut CheckerContext) -> bool {
        *self == Self::from_type_value(ann_type, ctx)
    }

    pub fn from_type_value(type_value: &TypeValue, ctx: &mut CheckerContext) -> Self {
        match type_value {
            TypeValue::TypeLiteral(type_literal) => match type_literal.literal {
                Literal::BooleanLiteral(_) => Self::Boolean,
                Literal::NumberLiteral(_) => Self::Number,
                Literal::NullLiteral(_) => Self::Null,
                Literal::StringLiteral(_) => Self::String,
            },
            TypeValue::KeywordType(keyword_type) => match keyword_type.kind {
                TypeKeyword::Boolean => Self::Boolean,
                TypeKeyword::String => Self::String,
                TypeKeyword::Number => Self::Number,
            },
            TypeValue::TypeReference(type_reference) => {
                match type_reference.type_name.name.as_bytes() {
                    b"Array" => todo!(),
                    b"Record" => {
                        let p = type_reference.type_params.as_ref();
                        if !p.is_some_and(|p| p.len() == 2) {
                            ctx.report_error(
                                "Invalid number of arguments".to_string(),
                                type_value.node().clone(),
                                ErrorSeverity::Critical,
                            );
                        }

                        let key_type = Self::from_type_value(&p.unwrap()[0], ctx);
                        let value_type = Self::from_type_value(&p.unwrap()[1], ctx);

                        Self::Object(Box::new(ObjectType {
                            key_type,
                            value_type,
                        }))
                    }
                    _ => todo!(),
                }
            }
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
        unfolded_type: Option<ExprType>,
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
    use super::{ExprType, ObjectType};
    use pretty_assertions::assert_eq;

    #[test]
    fn format_union() {
        use ExprType as T;
        let t = T::Union([T::String, T::Number, T::Null].to_vec());
        assert_eq!(t.to_string(), "string | number | null".to_string());
    }

    #[test]
    fn format_object() {
        use ExprType as T;
        let t = T::Object(Box::new(ObjectType {
            key_type: T::String,
            value_type: T::Union([T::Number, T::Null].to_vec()),
        }));
        assert_eq!(t.to_string(), "Record<string, number | null>".to_string());
    }
}

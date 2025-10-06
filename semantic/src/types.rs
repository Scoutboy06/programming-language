use lexer::TypeKeyword;
use parser::expressions::{types::AstType, Literal};

use crate::{errors::ErrorData, symbol::Symbol, CheckerContext, ErrorSeverity};

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    Unknown,
    Number,
    String,
    Boolean,
    Null,
    Array(Box<Self>),
    Object(Box<ObjectType>),
    Union(Vec<Self>),
    Function(Box<FunctionType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub key_type: ResolvedType,
    pub value_type: ResolvedType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Symbol>,
    pub display_ret_type: Option<AstType>,
    pub unfolded_ret_type: Option<ResolvedType>,
}

impl ResolvedType {
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

    pub fn includes(&self, t: &ResolvedType) -> bool {
        match self {
            Self::Union(u) => u.contains(t),
            _ => *self == *t,
        }
    }
}

impl std::fmt::Display for ResolvedType {
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

impl ResolvedType {
    pub fn from_ast_type(type_value: &AstType, ctx: &mut CheckerContext) -> Self {
        match type_value {
            AstType::TypeLiteral(type_literal) => match type_literal.literal {
                Literal::BooleanLiteral(_) => Self::Boolean,
                Literal::NumberLiteral(_) => Self::Number,
                Literal::NullLiteral(_) => Self::Null,
                Literal::StringLiteral(_) => Self::String,
            },
            AstType::KeywordType(keyword_type) => match keyword_type.kind {
                TypeKeyword::Boolean => Self::Boolean,
                TypeKeyword::String => Self::String,
                TypeKeyword::Number => Self::Number,
            },
            AstType::TypeReference(type_reference) => {
                match type_reference.type_name.name.as_bytes() {
                    b"Array" => todo!(),
                    b"Record" => {
                        let p = type_reference.type_params.as_ref();
                        if !p.is_some_and(|p| p.len() == 2) {
                            ctx.report_error(
                                ErrorData::InvalidNumberOfArguments {
                                    received: p.unwrap().len() as u8,
                                    expected: 2,
                                },
                                type_value.node().clone(),
                                ErrorSeverity::Critical,
                            );
                        }

                        let key_type = Self::from_ast_type(&p.unwrap()[0], ctx);
                        let value_type = Self::from_ast_type(&p.unwrap()[1], ctx);

                        Self::Object(Box::new(ObjectType {
                            key_type,
                            value_type,
                        }))
                    }
                    _ => todo!(),
                }
            }
            AstType::ArrayType(array_type) => {
                let left_type = Self::from_ast_type(&array_type.type_value, ctx);
                Self::Array(Box::new(left_type))
            }
            AstType::FnType(fn_type) => {
                let args: Vec<Symbol> = fn_type
                    .params
                    .iter()
                    .map(|arg| Symbol {
                        id: arg.identifier.name.clone(),
                        resolved_type: arg
                            .type_annotation
                            .as_ref()
                            .map(|ann| Self::from_ast_type(&ann.type_value, ctx)),
                        declared_at: arg.node.clone(),
                    })
                    .collect();

                Self::Function(Box::new(FunctionType {
                    args,
                    display_ret_type: todo!(),
                    unfolded_ret_type: todo!(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ObjectType, ResolvedType};
    use pretty_assertions::assert_eq;

    #[test]
    fn format_union() {
        use ResolvedType as T;
        let t = T::Union([T::String, T::Number, T::Null].to_vec());
        assert_eq!(t.to_string(), "string | number | null".to_string());
    }

    #[test]
    fn format_object() {
        use ResolvedType as T;
        let t = T::Object(Box::new(ObjectType {
            key_type: T::String,
            value_type: T::Union([T::Number, T::Null].to_vec()),
        }));
        assert_eq!(t.to_string(), "Record<string, number | null>".to_string());
    }

    #[test]
    fn format_array() {
        use ResolvedType as T;
        let t = T::Array(Box::new(ResolvedType::Union(
            [T::Number, T::Null, T::String].to_vec(),
        )));
        assert_eq!(t.to_string(), "(number | null | string)[]");
    }
}

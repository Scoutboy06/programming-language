use lexer::TypeKeyword;
use parser::expressions::{types::TypeValue, Literal};

use crate::{errors::ErrorData, symbol::Symbol, CheckerContext, ErrorSeverity};

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
    Function(Box<FunctionType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub key_type: ExprType,
    pub value_type: ExprType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Symbol>,
    pub display_ret_type: Option<TypeValue>,
    pub unfolded_ret_type: Option<ExprType>,
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

    pub fn includes(&self, t: &ExprType) -> bool {
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
                                ErrorData::InvalidNumberOfArguments {
                                    received: p.unwrap().len() as u8,
                                    expected: 2,
                                },
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
            TypeValue::ArrayType(array_type) => {
                let left_type = Self::from_type_value(&array_type.type_value, ctx);
                Self::Array(Box::new(left_type))
            }
            TypeValue::FnType(fn_type) => {
                let args: Vec<Symbol> = fn_type
                    .params
                    .iter()
                    .map(|arg| Symbol {
                        id: arg.identifier.name.clone(),
                        type_value: arg
                            .type_annotation
                            .as_ref()
                            .map(|ann| ann.type_value.clone()),
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

    #[test]
    fn format_array() {
        use ExprType as T;
        let t = T::Array(Box::new(ExprType::Union(
            [T::Number, T::Null, T::String].to_vec(),
        )));
        assert_eq!(t.to_string(), "(number | null | string)[]");
    }
}

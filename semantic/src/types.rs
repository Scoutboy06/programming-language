use lexer::TypeKeyword;
use parser::expressions::{types::TypeValue, Literal};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Number,
    String,
    Boolean,
    Null,
    Array(Vec<ExpressionType>),
    Object(Box<ObjectType>),
    Union(Vec<ExpressionType>),
    Function(()),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub key_type: ExpressionType,
    pub value_type: ExpressionType,
}

impl ExpressionType {
    pub fn matches(&self, ann_type: &TypeValue) -> bool {
        match ann_type {
            TypeValue::TypeLiteral(type_literal) => match type_literal.literal {
                Literal::BooleanLiteral(_) => matches!(self, Self::Boolean),
                Literal::NumberLiteral(_) => matches!(self, Self::Number),
                Literal::NullLiteral(_) => matches!(self, Self::Null),
                Literal::StringLiteral(_) => matches!(self, Self::String),
            },
            TypeValue::KeywordType(keyword_type) => match keyword_type.kind {
                TypeKeyword::String => matches!(self, Self::String),
                TypeKeyword::Number => matches!(self, Self::Number),
                TypeKeyword::Boolean => matches!(self, Self::Boolean),
            },
            TypeValue::TypeReference(_type_reference) => todo!(),
            TypeValue::ArrayType(_array_type) => todo!(),
        }
    }
}

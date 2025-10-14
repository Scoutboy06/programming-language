use crate::ast_types::patterns::{
    array_pattern::ArrayPattern, object_pattern::ObjectPattern, rest_element::RestElement,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    ArrayPattern(Box<ArrayPattern>),
    ObjectPattern(Box<ObjectPattern>),
    RestElement(Box<RestElement>),
}

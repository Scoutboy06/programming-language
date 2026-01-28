use parser_derive::FromVariants;

use crate::ast_types::{
    identifier::Identifier,
    node_objects::Node,
    patterns::{
        array_pattern::ArrayPattern, object_pattern::ObjectPattern, rest_element::RestElement,
    },
};

// es5
// interface Pattern <: Node { }
#[derive(Debug, PartialEq, Clone, FromVariants)]
pub enum Pattern {
    Identifier(Box<Identifier>),
    ArrayPattern(Box<ArrayPattern>),
    ObjectPattern(Box<ObjectPattern>),
    RestElement(Box<RestElement>),
}

impl Pattern {
    pub fn node(&self) -> &Node {
        match self {
            Self::Identifier(p) => &p.node,
            Self::ArrayPattern(p) => &p.node,
            Self::ObjectPattern(p) => &p.node,
            Self::RestElement(p) => &p.node,
        }
    }
}

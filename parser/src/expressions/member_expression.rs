use super::{Expression, Identifier};
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: MemberProperty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberProperty {
    Identifier(Identifier),
    Computed(ComputedProperty),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedProperty {
    pub node: Node,
    pub expression: Expression,
}

impl From<ComputedProperty> for MemberProperty {
    fn from(value: ComputedProperty) -> Self {
        MemberProperty::Computed(value)
    }
}

impl From<Identifier> for MemberProperty {
    fn from(value: Identifier) -> Self {
        MemberProperty::Identifier(value)
    }
}

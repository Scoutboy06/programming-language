use super::Expression;
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq, Clone)]
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

use super::Expression;
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: MemberProperty,
}

impl From<MemberExpression> for Expression {
    fn from(value: MemberExpression) -> Self {
        Expression::MemberExpression(Box::new(value))
    }
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

use super::Expression;
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Identifier,
}

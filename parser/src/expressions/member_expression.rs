use super::Expression;
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Identifier,
}

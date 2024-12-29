use crate::nodes::Node;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpression {
    pub node: Node,
    pub items: Vec<Expression>,
}

use super::Expression;
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

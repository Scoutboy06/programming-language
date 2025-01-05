use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Expression,
}

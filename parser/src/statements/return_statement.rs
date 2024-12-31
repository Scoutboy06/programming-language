use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub node: Node,
    pub value: Expression,
}

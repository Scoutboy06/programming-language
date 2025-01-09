use crate::{expressions::Expression, nodes::Node};

use super::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub node: Node,
    pub condition: Expression,
    pub body: BlockStatement,
}

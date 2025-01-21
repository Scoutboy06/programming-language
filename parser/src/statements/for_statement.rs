use parser_derive::Stmt;

use super::Statement;
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForStatement {
    pub node: Node,
    pub initializer: Statement,
    pub condition: Expression,
    pub update: Statement,
    pub body: Statement,
}

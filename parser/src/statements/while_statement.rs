use super::Statement;
use crate::{expressions::Expression, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct WhileStatement {
    pub node: Node,
    pub condition: Expression,
    pub body: Statement,
}

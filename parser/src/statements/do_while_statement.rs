use super::Statement;
use crate::{expressions::Expression, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct DoWhileStatement {
    pub node: Node,
    pub body: Statement,
    pub test: Expression,
}

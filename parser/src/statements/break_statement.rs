use super::Statement;
use crate::{expressions::Identifier, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct BreakStatement {
    pub node: Node,
    pub label: Option<Identifier>,
}

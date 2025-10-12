use super::Statement;
use crate::{expressions::Identifier, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ContinueStatement {
    pub node: Node,
    pub id: Option<Identifier>,
}

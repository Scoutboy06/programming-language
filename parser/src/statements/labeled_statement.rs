use super::Statement;
use crate::{expressions::Identifier, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct LabeledStatement {
    pub node: Node,
    pub label: Identifier,
    pub body: Statement,
}

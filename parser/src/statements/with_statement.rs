use super::Statement;
use crate::{expressions::Expression, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct WithStatement {
    pub node: Node,
    pub object: Expression,
    pub body: Statement,
}

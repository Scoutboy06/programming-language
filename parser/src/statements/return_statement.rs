use super::Statement;
use crate::{expressions::Expression, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct ReturnStatement {
    pub node: Node,
    pub value: Expression,
}

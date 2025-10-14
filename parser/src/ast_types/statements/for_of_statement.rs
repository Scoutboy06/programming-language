use parser_derive::Stmt;

use super::Statement;
use crate::{expressions::Expression, nodes::Node, statements::ForInOrOfLeft};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForOfStatement {
    pub node: Node,
    pub left: ForInOrOfLeft,
    pub right: Expression,
    pub body: Statement,
}

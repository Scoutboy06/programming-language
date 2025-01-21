use super::Statement;
use crate::nodes::Node;
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct BlockStatement {
    pub node: Node,
    pub statements: Vec<Statement>,
}

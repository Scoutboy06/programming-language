use super::Statement;
use crate::nodes::Node;
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct EmptyStatement {
    pub node: Node,
}

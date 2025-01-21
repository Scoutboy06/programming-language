use super::Statement;
use crate::nodes::Node;
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct Shebang {
    pub node: Node,
    pub value: String,
}

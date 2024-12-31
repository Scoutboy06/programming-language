use super::Statement;
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub node: Node,
    pub statements: Vec<Statement>,
}

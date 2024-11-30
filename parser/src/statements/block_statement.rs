use crate::nodes::Node;

use super::Statement;

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    node: Node,
    body: Vec<Statement>,
}

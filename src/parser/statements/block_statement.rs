use crate::parser::Node;

use super::{Statement, VariableDeclarator};

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    node: Node,
    body: Vec<Statement>,
}

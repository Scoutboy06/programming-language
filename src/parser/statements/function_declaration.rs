use crate::parser::Node;

use super::{BlockStatement, Identifier};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub node: Node,
    pub id: Identifier,
    pub is_expression: bool,
    pub is_generator: bool,
    pub is_async: bool,
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
}

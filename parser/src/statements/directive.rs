use parser_derive::Stmt;

use crate::{expressions::Literal, nodes::Node, statements::Statement};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct Directive {
    pub node: Node,
    pub expression: Literal,
    pub directive: String,
}

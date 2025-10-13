use super::Statement;
use crate::{nodes::Node, statements::Directive};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct BlockStatement {
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBodyBody {
    Directive(Directive),
    Statement(Statement),
}

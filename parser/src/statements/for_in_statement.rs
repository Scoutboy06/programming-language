use parser_derive::Stmt;

use super::Statement;
use crate::{
    expressions::Expression,
    nodes::{Node, Pattern},
    statements::VariableDeclaration,
};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForInStatement {
    pub node: Node,
    pub left: ForInLeft,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInLeft {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}
